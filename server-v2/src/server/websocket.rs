//! Websocket handling.

use futures::{select, FutureExt, SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::task::spawn_local;
use tokio_tungstenite::tungstenite::handshake::{
    headers::Headers,
    server::{ErrorResponse, Request},
};
use tokio_tungstenite::tungstenite::http::StatusCode;
use tokio_tungstenite::tungstenite::{Error as WsError, Message};
use tokio_tungstenite::{accept_hdr_async, WebSocketStream};

use std::cell::RefCell;
use std::io::Result as IOResult;
use std::net::SocketAddr;
use std::rc::Rc;

use crate::ecs::World;
use crate::resource::builtin::{PlayerCount, ShutdownFlag};
use crate::resource::socket::*;

macro_rules! headers {
	{ $( $hdr:literal : $val:expr ),* $(,)? } => {
		vec![ $( ( $hdr.to_string(), $val.to_string() ) ),* ]
	}
}

struct SavedHeaders {
    origin: Option<Vec<u8>>,
    forwarded: Option<Vec<u8>>,
    x_forwarded_for: Option<Vec<u8>>,
    sec_websocket_protocol: Option<Vec<u8>>,
    sec_websocket_version: Option<u32>,
}

impl SavedHeaders {
    fn into_event(self, tx: UnboundedSender<SocketMessage>, id: SocketId) -> ConnectEvent {
        ConnectEvent {
            origin: self.origin,
            forwarded: self.forwarded,
            x_forwarded_for: self.x_forwarded_for,
            sec_websocket_protocol: self.sec_websocket_protocol,
            sec_websocket_version: self.sec_websocket_version,

            socket: SocketWriter::new(tx),
            socketid: id,
        }
    }
}

/// Listens on the given socket for new websocket connections.
///
/// In addition, it will answer number-of-player queries to
/// the server when the request is not a valid websocket upgrade.
pub async fn websocket_listener(world: Rc<RefCell<World>>, port: SocketAddr) {
    if let Err(e) = _websocket_listener(Rc::clone(&world), port).await {
        error!(
            "Websocket listener on port {} closed with error: {}",
            port, e
        );
    }

    let world = world.borrow_mut();
    let mut flag = world.fetch_resource_mut::<ShutdownFlag>();
    flag.shutdown();
}

async fn _websocket_listener(world: Rc<RefCell<World>>, port: SocketAddr) -> IOResult<()> {
    let mut listener = TcpListener::bind(port).await?;

    loop {
        let (stream, remoteaddr) = listener.accept().await?;
        let world = Rc::clone(&world);

        spawn_local(drive_socket(world, stream, remoteaddr));
    }
}

async fn drive_socket(world: Rc<RefCell<World>>, stream: TcpStream, remote: SocketAddr) {
    if let Err(e) = _drive_socket(&world, stream, remote).await {
        debug!(
            "Connection from address {} closed due to error: {}",
            remote, e
        );
    }

    // TODO: Close stream here?
}

async fn _drive_socket(
    world: &RefCell<World>,
    stream: TcpStream,
    _remote: SocketAddr,
) -> Result<(), WsError> {
    let (tx, rx) = unbounded_channel();
    let mut saved = None;

    let cb = |req: &Request| -> Result<Option<_>, _> {
        let headers = &req.headers;

        if req.path != "/" {
            return Err(respond_not_found());
        }

        // For backwards compatibility don't support custom websocket
        // protocol versions. This means that it we want to upgrade to
        // a new prototocol it can be done in a way that is backwards
        // compatible.
        if headers.find_first("Sec-Websocket-Protocol").is_some() {
            return Err(respond_bad_protocol());
        }
        if headers.find_first("Sec-Websocket-Version").is_some() {
            return Err(respond_bad_protocol());
        }

        if !is_ws_upgrade(headers) {
            return Err(respond_playercount(world));
        }

        saved = Some(SavedHeaders {
            origin: headers.find_first("Origin").map(|x| x.to_vec()),
            forwarded: headers.find_first("Forwarded").map(|x| x.to_vec()),
            x_forwarded_for: headers.find_first("X-Forwarded-For").map(|x| x.to_vec()),
            sec_websocket_protocol: headers
                .find_first("Sec-Websocket-Protocol")
                .map(|x| x.to_vec()),
            sec_websocket_version: headers
                .find_first("Sec-Websocket-Version")
                .and_then(|x| std::str::from_utf8(x).ok()?.parse().ok()),
        });

        Ok(None)
    };

    let mut wsstream = match accept_hdr_async(stream, cb).await {
        Ok(stream) => stream,
        Err(_) => return Ok(()),
    };

    let socketid = SocketId::new();

    {
        let saved = saved.unwrap();

        let world = world.borrow();
        let mut channel = world.fetch_resource_mut::<OnConnect>();

        channel.single_write(saved.into_event(tx, socketid));
    }

    let res = drive_websocket(&mut wsstream, rx, world, socketid).await;

    let world = world.borrow();
    let mut channel = world.fetch_resource_mut::<OnClose>();

    channel.single_write(CloseEvent { socket: socketid });

    res
}

async fn drive_websocket(
    stream: &mut WebSocketStream<TcpStream>,
    mut rx: UnboundedReceiver<SocketMessage>,
    world: &RefCell<World>,
    id: SocketId,
) -> Result<(), WsError> {
    enum Task {
        Read(Message),
        Write(SocketMessage),
    }

    loop {
        let task = select! {
            msg = stream.next().fuse() => match msg {
                Some(msg) => Task::Read(msg?),
                None => return Ok(())
            },
            res = rx.recv().fuse() => match res {
                Some(msg) => Task::Write(msg),
                None => return Ok(())
            }
        };

        match task {
            Task::Write(msg) => {
                let msg = match &msg {
                    SocketMessage::Data(data) => Message::Binary(data.to_vec()),
                    SocketMessage::Close => Message::Close(None),
                };

                stream.send(msg).await?;
            }
            Task::Read(msg) => {
                let world = world.borrow();
                let mut messages = world.fetch_resource_mut::<OnMessage>();

                match msg {
                    Message::Binary(data) => {
                        messages.single_write(MessageEvent { socket: id, data })
                    }
                    Message::Text(text) => messages.single_write(MessageEvent {
                        socket: id,
                        data: text.into_bytes(),
                    }),
                    Message::Ping(data) => {
                        drop(messages);
                        drop(world);

                        stream.send(Message::Pong(data)).await?;
                    }
                    Message::Pong(_) => {}
                    Message::Close(_) => break,
                }
            }
        }
    }

    Ok(())
}

/// Send error response for a protocol we don't support.
fn respond_bad_protocol() -> ErrorResponse {
    ErrorResponse {
        error_code: StatusCode::from_u16(400).unwrap(),
        headers: Some(headers! {
            "Server": "AIRMASH",
            "Content-Type": "text/plain"
        }),
        body: Some("Unsupported Protocol Version".to_string()),
    }
}

fn respond_not_found() -> ErrorResponse {
    ErrorResponse {
        error_code: StatusCode::from_u16(404).unwrap(),
        headers: Some(headers! {
            "Server": "AIRMASH",
            "Content-Type": "text/plain"
        }),
        body: Some("Not Found".to_string()),
    }
}

fn respond_playercount(world: &RefCell<World>) -> ErrorResponse {
    let nplayers = world.borrow().fetch_resource::<PlayerCount>().0;

    ErrorResponse {
        error_code: StatusCode::from_u16(200).unwrap(),
        headers: Some(headers! {
            "Server": "AIRMASH",
            "Content-Type": "application/json"
        }),
        body: Some(format!(r#"{{ "players": {}" }}"#, nplayers)),
    }
}

fn is_ws_upgrade(headers: &Headers) -> bool {
    if !headers.header_is("Upgrade", "websocket") {
        return false;
    }

    if !headers.header_is("Connection", "Upgrade") {
        return false;
    }

    if headers.find_first("Sec-Websocket-Key").is_none() {
        return false;
    }

    true
}

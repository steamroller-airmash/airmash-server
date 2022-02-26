//! Raw networking interfaces.
//!
//! This module has the raw types for interacting with the network endpoint of
//! the server. Usually you'll want to use the helper methods on [`AirmashGame`]
//! instead of interacting directly with these types.
//!
//! [`AirmashGame`]: crate::AirmashGame

use std::{
  collections::HashMap,
  fmt,
  io::ErrorKind,
  net::SocketAddr,
  sync::atomic::{AtomicBool, AtomicUsize, Ordering},
  sync::Arc,
  thread::JoinHandle,
};

use crossbeam_channel::{unbounded, Receiver, Sender};
use futures_util::{sink::SinkExt, stream::StreamExt};
use hecs::Entity;
use httparse::{Status, EMPTY_HEADER};
use tokio::{
  io::AsyncReadExt,
  net::{TcpListener, TcpStream},
};
use tokio::{
  io::AsyncWriteExt,
  sync::mpsc::{unbounded_channel, UnboundedSender as AsyncSender},
};
use tokio_tungstenite::{
  tungstenite::{self, Message},
  WebSocketStream,
};

use crate::mock::MockConnectionEndpoint;

// TODO: This shouldn't be a global. It should instead be an Arc/Resource pair.
pub(crate) static NUM_PLAYERS: AtomicUsize = AtomicUsize::new(0);

/// Unique ID for a remote connection.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConnectionId(pub(crate) usize);

impl ConnectionId {
  pub fn id(&self) -> usize {
    self.0
  }
}

pub(crate) struct ConnectionData {
  pub(crate) send: AsyncSender<Arc<Vec<u8>>>,
  pub(crate) addr: SocketAddr,
}

pub(crate) enum InternalEvent {
  Opened(ConnectionData),
  Data(Vec<u8>),
  Closed,
}

pub(crate) enum ConnectionEvent {
  Opened,
  Data(Vec<u8>),
  Closed(Option<Entity>),
}

/// Interface for communicating with the networking side of the server.
///
/// The only way to initialize the server here is by calling
/// [`AirmashGame::with_network`] so you usually won't need to interact with
/// this struct directly. The one exception is for test cases in which case you
/// want to call [`disconnected`] to get a mock connection endpoint that can be
/// used to send messages without having to open up an actual server port.
///
/// [`AirmashGame::with_network`]: crate::AirmashGame::with_network
/// [`disconnected`]: crate::network::ConnectionMgr::disconnected
pub struct ConnectionMgr {
  conns: HashMap<ConnectionId, ConnectionData>,
  primary: HashMap<Entity, ConnectionId>,
  known: HashMap<ConnectionId, Entity>,

  recv: Receiver<(ConnectionId, InternalEvent)>,
  handle: Option<JoinHandle<()>>,
  shutdown: Arc<AtomicBool>,
}

impl ConnectionMgr {
  pub(crate) fn with_server(addr: SocketAddr, shutdown: Arc<AtomicBool>) -> Self {
    let (evttx, evtrx) = unbounded();

    let handle = std::thread::spawn({
      let shutdown = Arc::clone(&shutdown);
      move || server_thread(addr, evttx, shutdown)
    });

    Self {
      conns: Default::default(),
      primary: Default::default(),
      known: Default::default(),
      recv: evtrx,
      handle: Some(handle),
      shutdown,
    }
  }

  /// Create a connection manager with no associated server. This is meant to
  /// allow for testing and generally shouldn't be used otherwise.
  pub fn disconnected() -> (Self, MockConnectionEndpoint) {
    let (tx, rx) = unbounded();

    let me = Self {
      conns: Default::default(),
      primary: Default::default(),
      known: Default::default(),
      recv: rx,
      handle: None,
      shutdown: Arc::new(AtomicBool::new(false)),
    };
    let mock = MockConnectionEndpoint::new(tx);

    (me, mock)
  }

  pub fn send_to_conn(&mut self, conn: ConnectionId, message: Arc<Vec<u8>>) {
    if let Some(data) = self.conns.get_mut(&conn) {
      let _ = data.send.send(message);
    }
  }

  pub fn send_to(&mut self, ent: Entity, message: Arc<Vec<u8>>) {
    if let Some(&conn) = self.primary.get(&ent) {
      self.send_to_conn(conn, message);
    }
  }

  pub fn socket_addr(&self, conn: ConnectionId) -> Option<SocketAddr> {
    self.conns.get(&conn).map(|x| x.addr)
  }

  pub fn associate(&mut self, ent: Entity, conn: ConnectionId) {
    self.known.insert(conn, ent);
    self.primary.entry(ent).or_insert_with(|| conn);
  }

  pub fn associated(&self, conn: ConnectionId) -> Option<Entity> {
    self.known.get(&conn).copied()
  }

  pub fn mark_primary(&mut self, ent: Entity, conn: ConnectionId) {
    self.primary.insert(ent, conn);
  }

  pub(crate) fn next_packet(&mut self) -> Option<(ConnectionId, ConnectionEvent)> {
    let (conn, evt) = self.recv.try_recv().ok()?;

    Some((
      conn,
      match evt {
        InternalEvent::Opened(data) => {
          self.conns.insert(conn, data);
          ConnectionEvent::Opened
        }
        InternalEvent::Data(data) => ConnectionEvent::Data(data),
        InternalEvent::Closed => ConnectionEvent::Closed(match self.known.remove(&conn) {
          Some(ent) => match self.primary.get(&ent) {
            Some(&econn) if econn == conn => {
              self.primary.remove(&ent);
              Some(ent)
            }
            _ => None,
          },
          _ => None,
        }),
      },
    ))
  }
}

impl Drop for ConnectionMgr {
  fn drop(&mut self) {
    self.shutdown.store(true, Ordering::Relaxed);
    if let Some(handle) = self.handle.take() {
      let _ = handle.join();
    }
  }
}

fn server_thread(
  addr: SocketAddr,
  send: Sender<(ConnectionId, InternalEvent)>,
  shutdown: Arc<AtomicBool>,
) {
  use tokio::runtime::Builder;

  #[cfg(feature = "mt-network")]
  let mut builder = Builder::new_multi_thread();
  #[cfg(not(feature = "mt-network"))]
  let mut builder = Builder::new_current_thread();

  let rt = builder
    .enable_all()
    .build()
    .expect("Failed to initialize tokio runtime");

  if let Err(e) = rt.block_on(run_server(addr, send, shutdown.clone())) {
    error!("Websocket server shutting down with error: {}", e);
  }

  shutdown.store(true, Ordering::Relaxed);
}

async fn run_server(
  addr: SocketAddr,
  send: Sender<(ConnectionId, InternalEvent)>,
  shutdown: Arc<AtomicBool>,
) -> std::io::Result<()> {
  let socket = TcpListener::bind(&addr).await?;
  info!("Listening on {}", addr);

  let mut connid: usize = 0;

  while !shutdown.load(Ordering::Relaxed) {
    let send = send.clone();
    let conn = ConnectionId(connid);
    connid += 1;

    tokio::select! {
      res = socket.accept() => {
        let (stream, addr) = res?;

        tokio::spawn(async move {
          let _ = run_connection(stream, addr, conn, &send).await;
          let _ = send.send((conn, InternalEvent::Closed));
        });
      }
      _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => ()
    }
  }

  Ok(())
}

async fn run_connection(
  stream: TcpStream,
  addr: SocketAddr,
  conn: ConnectionId,
  events: &Sender<(ConnectionId, InternalEvent)>,
) -> std::io::Result<()> {
  let addr = stream.peer_addr().unwrap_or(addr);

  let mut ws_stream = match websocket_handshake(stream, &addr).await {
    Ok(Some(stream)) => stream,
    Ok(None) => return Ok(()),
    Err(e) => {
      warn!("{}", e);
      return Ok(());
    }
  };

  let (tx, mut rx) = unbounded_channel();

  if let Err(_) = events.send((
    conn,
    InternalEvent::Opened(ConnectionData { send: tx, addr }),
  )) {
    return Ok(());
  }

  loop {
    tokio::select! {
      read = ws_stream.next() => {
        let msg = match read {
          Some(Ok(read)) => read,
          _ => return Ok(())
        };

        if msg.is_binary() || msg.is_text() {
          if let Err(_) = events.send((conn, InternalEvent::Data(msg.into_data()))) {
            return Ok(())
          }
        } else {
          match msg {
            Message::Ping(data) => {
              let _ =  ws_stream.send(Message::Pong(data)).await;
            }
            Message::Pong(_) => (),
            Message::Close(_) => return Ok(()),
            _ => unreachable!()
          }
        }
      }
      write = rx.recv() => {
        let write = match write {
          Some(write) => write,
          None => return Ok(())
        };

        let data = match Arc::try_unwrap(write) {
          Ok(data) => data,
          Err(e) => (*e).clone()
        };

        if let Err(_) = ws_stream.send(Message::binary(data)).await {
          return Ok(())
        }
      }
    }
  }
}

impl fmt::Display for ConnectionId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

fn log_request(addr: &SocketAddr, code: u16, request: &httparse::Request) {
  use crate::util::escapes::StringEscape;

  info!(
    "{} - \"{} {} HTTP/1.{}\" {} \"{}\"",
    addr.ip(),
    request.method.unwrap_or("\"\""),
    request.path.unwrap_or("\"\""),
    request.version.unwrap_or(0),
    code,
    get_header(request, "User-Agent")
      .unwrap_or_default()
      .escaped(),
  );
}

fn get_header<'h, 'b>(request: &httparse::Request<'h, 'b>, name: &str) -> Option<&'b [u8]> {
  request
    .headers
    .iter()
    .filter(|h| h.name.eq_ignore_ascii_case(name))
    .map(|h| h.value)
    .next()
}

fn has_header(request: &httparse::Request, name: &str, value: &str) -> bool {
  request
    .headers
    .iter()
    .filter(|h| h.name.eq_ignore_ascii_case(name) && h.value.eq_ignore_ascii_case(value.as_bytes()))
    .next()
    .is_some()
}

async fn websocket_handshake(
  mut stream: TcpStream,
  addr: &SocketAddr,
) -> std::io::Result<Option<WebSocketStream<TcpStream>>> {
  use httparse::Request;
  use std::io::Error;
  use tungstenite::error::ProtocolError;
  use tungstenite::protocol::Role;

  const BAD_REQUEST: &[u8] = b"HTTP/1.0 400 Bad Request\r\n\r\n";
  const BAD_PROTOCOL: &[u8] = b"HTTP/1.0 405 Method Not Allowed\r\n\r\n";
  const NOT_FOUND: &[u8] = b"HTTP/1.0 404 Not Found\r\n\r\n";

  let response = format!(
    "HTTP/1.0 200 OK\r\nContent-Type: application/json; charset=utf=8\r\n\r\n\
    {{\"players\":{}}}\n",
    NUM_PLAYERS.load(Ordering::Relaxed)
  )
  .into_bytes();

  let mut buf = Vec::new();

  loop {
    stream.read_buf(&mut buf).await?;

    let mut headers = [EMPTY_HEADER; 32];
    let mut request = Request::new(&mut headers);

    let bytes = match request.parse(&buf) {
      Ok(Status::Complete(bytes)) => bytes,
      Ok(Status::Partial) => continue,
      Err(e) => {
        log_request(addr, 400, &request);
        stream.write_all(BAD_REQUEST).await?;
        return Err(Error::new(ErrorKind::Other, e));
      }
    };

    if request.method != Some("GET") {
      log_request(addr, 405, &request);
      stream.write_all(BAD_PROTOCOL).await?;
      return Err(Error::new(ErrorKind::Other, ProtocolError::WrongHttpMethod));
    }

    if request.path != Some("/") {
      log_request(addr, 404, &request);
      stream.write_all(NOT_FOUND).await?;
      return Err(Error::new(ErrorKind::NotFound, "Invalid request path"));
    }

    let has_connection_upgrade = request
      .headers
      .iter()
      .filter(|h| h.name.eq_ignore_ascii_case("Connection"))
      .any(|h| {
        h.value
          .split(|&c| c == b' ' || c == b',')
          .any(|p| p.eq_ignore_ascii_case(b"Upgrade"))
      });
    let has_upgrade_websocket = has_header(&request, "Upgrade", "websocket");

    if !has_connection_upgrade && !has_upgrade_websocket {
      log_request(addr, 200, &request);
      stream.write_all(&response).await?;
      return Ok(None);
    }

    if !has_connection_upgrade
      || !has_upgrade_websocket
      || !has_header(&request, "Sec-Websocket-Version", "13")
    {
      log_request(addr, 400, &request);
      stream.write_all(&response).await?;
      return Err(Error::new(
        ErrorKind::Other,
        ProtocolError::MissingConnectionUpgradeHeader,
      ));
    }

    let key = match get_header(&request, "Sec-Websocket-Key") {
      Some(key) => key,
      None => {
        log_request(addr, 400, &request);
        stream.write_all(&response).await?;
        return Err(Error::new(
          ErrorKind::Other,
          ProtocolError::MissingSecWebSocketKey,
        ));
      }
    };

    let response = format!(
      "HTTP/1.1 101 Switching Protocols\r\n\
      Connection: Upgrade\r\n\
      Upgrade: websocket\r\n\
      Sec-WebSocket-Accept: {}\r\n\
      \r\n",
      tungstenite::handshake::derive_accept_key(key)
    );
    log_request(addr, 101, &request);
    stream.write_all(response.as_bytes()).await?;

    buf.drain(..bytes);
    break;
  }

  let wss = WebSocketStream::from_partially_read(stream, buf, Role::Server, None).await;
  Ok(Some(wss))
}

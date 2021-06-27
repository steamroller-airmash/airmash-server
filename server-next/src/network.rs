use std::{
  collections::HashMap,
  fmt,
  net::{IpAddr, SocketAddr},
  sync::atomic::{AtomicBool, AtomicUsize, Ordering},
  sync::Arc,
  thread::JoinHandle,
};

use airmash_protocol::{ClientPacket, ServerPacket};
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures_util::{sink::SinkExt, stream::StreamExt};
use hecs::Entity;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender as AsyncSender};
use tokio::{
  net::{TcpListener, TcpStream},
  sync::mpsc::UnboundedReceiver,
};
use tokio_tungstenite::tungstenite::{
  handshake::server::{ErrorResponse, Response},
  http::Request,
  Message,
};

pub static NUM_PLAYERS: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConnectionId(usize);

impl ConnectionId {
  pub fn id(&self) -> usize {
    self.0
  }
}

struct ConnectionData {
  send: AsyncSender<Vec<u8>>,

  addr: SocketAddr,
}

enum InternalEvent {
  Opened(ConnectionData),
  Data(Vec<u8>),
  Closed,
}

pub enum ConnectionEvent {
  Opened,
  Data(Vec<u8>),
  Closed(Option<Entity>),
}

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

  pub fn send_to_conn(&mut self, conn: ConnectionId, message: Vec<u8>) {
    if let Some(data) = self.conns.get_mut(&conn) {
      let _ = data.send.send(message);
    }
  }

  pub fn send_to(&mut self, ent: Entity, message: Vec<u8>) {
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

  pub fn next_packet(&mut self) -> Option<(ConnectionId, ConnectionEvent)> {
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
          Some(ent) if self.primary.get(&ent) == Some(&conn) => Some(ent),
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

pub struct MockReceiver {
  rx: UnboundedReceiver<Vec<u8>>,
  conn: ConnectionId,
}

impl MockReceiver {
  pub fn conn(&self) -> ConnectionId {
    self.conn
  }

  pub fn next_raw(&mut self) -> Option<Vec<u8>> {
    use std::task::{Context, Poll};

    let waker = futures_util::task::noop_waker_ref();
    let mut ctx = Context::from_waker(waker);
    match self.rx.poll_recv(&mut ctx) {
      Poll::Pending => None,
      Poll::Ready(x) => x,
    }
  }

  pub fn next_packet(&mut self) -> Option<ServerPacket> {
    let data = self.next_raw()?;
    let packet = crate::protocol::v5::deserialize(&data).unwrap_or_else(|e| {
      panic!(
        "Server sent invalid packet. Error is {}. Packet is:\n  {:?}",
        e, data
      )
    });

    println!("{}: {:#?}", self.conn.id(), packet);

    Some(packet)
  }
}

pub struct MockConnectionEndpoint {
  sender: Sender<(ConnectionId, InternalEvent)>,
  nextid: usize,
}

impl MockConnectionEndpoint {
  fn new(sender: Sender<(ConnectionId, InternalEvent)>) -> Self {
    Self { sender, nextid: 0 }
  }

  pub fn open(&mut self) -> (ConnectionId, MockReceiver) {
    let conn = ConnectionId(self.nextid);
    self.nextid += 1;

    let (tx, rx) = unbounded_channel();

    self
      .sender
      .send((
        conn,
        InternalEvent::Opened(ConnectionData {
          send: tx,
          addr: SocketAddr::new(IpAddr::from([0; 4]), 0),
        }),
      ))
      .expect("Network event channel is closed");

    let recv = MockReceiver { rx, conn };

    (conn, recv)
  }

  pub fn send_raw(&mut self, conn: ConnectionId, data: Vec<u8>) {
    self
      .sender
      .send((conn, InternalEvent::Data(data)))
      .expect("Network event channel is closed");
  }

  pub fn send(&mut self, conn: ConnectionId, packet: impl Into<ClientPacket>) {
    let data = crate::protocol::v5::serialize(&packet.into()).expect("Failed to serialize packet");
    self.send_raw(conn, data);
  }

  pub fn close(&mut self, conn: ConnectionId) {
    let _ = self.sender.send((conn, InternalEvent::Closed));
  }
}

fn server_thread(
  addr: SocketAddr,
  send: Sender<(ConnectionId, InternalEvent)>,
  shutdown: Arc<AtomicBool>,
) {
  use tokio::runtime::Builder;

  let rt = Builder::new_current_thread()
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

  let res = tokio_tungstenite::accept_hdr_async(stream, |request: &Request<_>, response| {
    let headers = request.headers();

    let upgrade = match headers.get("Upgrade") {
      Some(upgrade) => upgrade,
      None => return Err(generate_status_response()),
    };

    if upgrade != "websocket" {
      return Err(generate_status_response());
    }

    Ok(response)
  })
  .await;

  let mut ws_stream = match res {
    Ok(stream) => stream,
    Err(_) => return Ok(()),
  };

  info!("New client connected from {}", addr);

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

        if let Err(_) = ws_stream.send(Message::Binary(write)).await {
          return Ok(())
        }
      }
    }
  }
}

fn generate_status_response() -> ErrorResponse {
  Response::builder()
    .status(200)
    .header("Content-Type", "application/json; charset=utf-8")
    .body(Some(format!(
      "{{\"players\":{}}}",
      NUM_PLAYERS.load(Ordering::Relaxed)
    )))
    .expect("Failed to generate status response")
}

impl fmt::Display for ConnectionId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

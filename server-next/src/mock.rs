use crate::protocol::{client as c, ClientPacket, ServerPacket};
use crate::{network::*, AirmashGame};

use airmash_protocol::KeyCode;
use crossbeam_channel::Sender;
use hecs::Entity;
use std::{
  net::{IpAddr, SocketAddr},
  sync::Arc,
  time::{Duration, Instant},
};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

/// Mock connection for testing purposes.
///
/// This allows for sending packets to the server without having to go through a
/// network connection. This is essential for ensuring that tests don't
/// interfere with each other.
pub struct MockConnection {
  tx: Sender<(ConnectionId, InternalEvent)>,
  rx: UnboundedReceiver<Arc<Vec<u8>>>,
  conn: ConnectionId,
  closed: bool,

  seq: u32,
}

impl MockConnection {
  fn new(
    tx: Sender<(ConnectionId, InternalEvent)>,
    rx: UnboundedReceiver<Arc<Vec<u8>>>,
    conn: ConnectionId,
  ) -> Self {
    Self {
      tx,
      rx,
      conn,
      closed: false,

      seq: 0,
    }
  }

  pub fn conn(&self) -> ConnectionId {
    self.conn
  }

  pub fn next_raw(&mut self) -> Option<Vec<u8>> {
    use std::task::{Context, Poll};

    let waker = futures_util::task::noop_waker_ref();
    let mut ctx = Context::from_waker(waker);
    match self.rx.poll_recv(&mut ctx) {
      Poll::Pending => None,
      Poll::Ready(Some(x)) => Some(match Arc::try_unwrap(x) {
        Ok(x) => x,
        Err(arc) => (*arc).clone(),
      }),
      Poll::Ready(None) => None,
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

  pub fn packets<'a>(&'a mut self) -> impl Iterator<Item = ServerPacket> + 'a {
    std::iter::from_fn(move || self.next_packet())
  }

  pub fn send_raw(&mut self, data: Vec<u8>) {
    assert!(!self.closed, "Tried to send to a closed client");

    self
      .tx
      .send((self.conn, InternalEvent::Data(data)))
      .expect("Network event channel is closed");
  }

  pub fn send(&mut self, packet: impl Into<ClientPacket>) {
    let data = crate::protocol::v5::serialize(&packet.into()).expect("Failed to serialize packet");
    self.send_raw(data);
  }

  pub fn close(&mut self) {
    if std::mem::replace(&mut self.closed, false) {
      let _ = self.tx.send((self.conn, InternalEvent::Closed));
    }
  }
}

impl MockConnection {
  /// Log in to the game and read the assigned entity from the game state. Note
  /// that this method will run one iteration of the game loop.
  pub fn login(&mut self, name: &str, game: &mut TestGame) -> Entity {
    self.send_login(name);
    game.run_once();
    self.wait_for_login(game)
  }

  /// Send a default login packet. This should be good enough for most test use
  /// cases.
  pub fn send_login(&mut self, name: &str) {
    self.send(c::Login {
      protocol: 5,
      session: Default::default(),
      name: name.into(),
      horizon_x: 4000,
      horizon_y: 4000,
      flag: "UN".into(),
    });
  }

  pub fn send_key(&mut self, key: KeyCode, state: bool) {
    self.send(c::Key {
      key,
      state,
      seq: self.seq,
    });

    self.seq += 1;
  }

  pub fn send_command(&mut self, command: &str, data: &str) {
    self.send(c::Command {
      com: command.into(),
      data: data.into(),
    });
  }

  /// Wait for the login packet and return the associated entity.
  pub fn wait_for_login(&mut self, game: &mut TestGame) -> Entity {
    let packet = self.next_packet().expect("No packets available.");

    let id = match packet {
      ServerPacket::Login(login) => login.id,
      _ => panic!("Expected Login packet, got: {:#?}", packet),
    };

    game
      .find_entity_by_id(id)
      .unwrap_or_else(|| panic!("Found no entity with id {}", id))
  }
}

impl Drop for MockConnection {
  fn drop(&mut self) {
    self.close();
  }
}

/// Mock connection endpoint for testing purposes.
///
/// This allows for creating new connections without needing open an actual
/// server on a socket.
pub struct MockConnectionEndpoint {
  sender: Sender<(ConnectionId, InternalEvent)>,
  nextid: usize,
}

impl MockConnectionEndpoint {
  pub(crate) fn new(sender: Sender<(ConnectionId, InternalEvent)>) -> Self {
    Self { sender, nextid: 0 }
  }

  pub fn open(&mut self) -> MockConnection {
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

    MockConnection::new(self.sender.clone(), rx, conn)
  }
}

/// Game wrapper for common tasks that need to be done as part of a test.
///
/// This covers things like ensuring that the server startup event is properly
/// dispatched and properly incrementing the frame time as we go. It also
/// handles setting the current time for the game as the game does not run in
/// real-time within tests.
pub struct TestGame {
  game: AirmashGame,
  now: Instant,
}

impl TestGame {
  fn frame_time() -> Duration {
    Duration::from_secs_f64(1.0 / 60.0)
  }

  /// Create a new server instance and corresponding connection endpoint.
  pub fn new() -> (Self, MockConnectionEndpoint) {
    use crate::event::ServerStartup;

    let mut game = AirmashGame::with_test_defaults();
    let (connmgr, mock) = ConnectionMgr::disconnected();
    game.resources.insert(connmgr);

    let start = game.start_time();

    let mut tg = TestGame {
      game,
      now: start + Duration::from_secs(60),
    };

    tg.dispatch(ServerStartup);

    (tg, mock)
  }

  /// Run the game for one main loop iteration.
  pub fn run_once(&mut self) {
    self.game.run_once(self.now);
    self.now += Self::frame_time();
  }

  /// Run `count` iterations of the main loop.
  pub fn run_count(&mut self, count: usize) {
    for _ in 0..count {
      self.run_once();
    }
  }

  /// Run the main loop for `duration` simulated time.
  ///
  /// Note that the main loop runs at 60 FPS so that will be used to determine
  /// how many iterations are run.
  pub fn run_for(&mut self, duration: Duration) {
    let target = self.now + duration;

    while self.now < target {
      self.run_once();
    }
  }

  /// Skip some amount of time without running any time steps.
  pub fn skip(&mut self, duration: Duration) {
    let target = self.now + duration;

    while self.now < target {
      self.now += Self::frame_time();
    }
  }
}

impl std::ops::Deref for TestGame {
  type Target = AirmashGame;

  fn deref(&self) -> &Self::Target {
    &self.game
  }
}

impl std::ops::DerefMut for TestGame {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.game
  }
}

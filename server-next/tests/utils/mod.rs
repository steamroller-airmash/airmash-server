use std::time::{Duration, Instant};

use airmash_protocol::ServerPacket;
use airmash_server::network::MockReceiver;
use server::event::ServerStartup;
use server::network::MockConnectionEndpoint;
use server::protocol::client as c;
use server::{AirmashWorld, ConnectionMgr};

pub fn create_login_packet(name: &str) -> c::Login {
  c::Login {
    protocol: 5,
    session: Default::default(),
    name: name.into(),
    horizon_x: 4000,
    horizon_y: 4000,
    flag: "UN".into(),
  }
}

pub fn get_login_id(mock: &mut MockReceiver) -> u16 {
  let packet = mock.next_packet().expect("No packets available");

  match packet {
    ServerPacket::Login(login) => login.id,
    _ => panic!("Expected Login packet, got: {:#?}", packet),
  }
}

pub fn create_mock_server() -> (GameWrapper, MockConnectionEndpoint) {
  let mut game = AirmashWorld::with_test_defaults();
  let (connmgr, mock) = ConnectionMgr::disconnected();
  game.resources.insert(connmgr);

  let mut wrapper = GameWrapper::new(game);
  wrapper.run_count(60);

  (wrapper, mock)
}

pub struct GameWrapper {
  game: AirmashWorld,
  now: Instant,
  started: bool,
}

impl GameWrapper {
  fn new(game: AirmashWorld) -> Self {
    Self {
      game,
      now: Instant::now(),
      started: false,
    }
  }

  pub fn run_once(&mut self) {
    if !self.started {
      self.game.dispatch(ServerStartup);
      self.started = true;
    }

    self.game.run_once(self.now);
    self.now += Duration::from_secs_f64(1.0 / 60.0);
  }

  pub fn run_count(&mut self, count: usize) {
    if !self.started {
      self.game.dispatch(ServerStartup);
      self.started = true;
    }

    for _ in 0..count {
      self.game.run_once(self.now);
      self.now += Duration::from_secs_f64(1.0 / 60.0);
    }
  }
}

impl std::ops::Deref for GameWrapper {
  type Target = AirmashWorld;

  fn deref(&self) -> &Self::Target {
    &self.game
  }
}

impl std::ops::DerefMut for GameWrapper {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.game
  }
}

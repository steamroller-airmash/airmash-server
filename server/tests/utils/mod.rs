use airmash_protocol::ServerPacket;
use server::protocol::client as c;
use server::test::{MockConnection, MockConnectionEndpoint, TestGame};

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

pub fn get_login_id(mock: &mut MockConnection) -> u16 {
  let packet = mock.next_packet().expect("No packets available");

  match packet {
    ServerPacket::Login(login) => login.id,
    _ => panic!("Expected Login packet, got: {:#?}", packet),
  }
}

pub fn create_mock_server() -> (TestGame, MockConnectionEndpoint) {
  TestGame::new()
}

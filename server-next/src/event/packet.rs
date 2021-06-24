
use hecs::Entity;
use crate::{network::ConnectionId, protocol::client as c};

#[derive(Copy, Clone, Debug)]
pub struct PacketEvent<P> {
  pub conn: ConnectionId,
  pub entity: Entity,
  pub packet: P
}

#[derive(Clone, Debug)]
pub struct LoginEvent {
  pub conn: ConnectionId,
  pub packet: c::Login
}

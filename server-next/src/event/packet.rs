use crate::{network::ConnectionId, protocol::client as c};
use hecs::Entity;

#[derive(Copy, Clone, Debug)]
pub struct PacketEvent<P> {
  pub conn: ConnectionId,
  pub entity: Entity,
  pub packet: P,
}

#[derive(Clone, Debug)]
pub struct LoginEvent {
  pub conn: ConnectionId,
  pub packet: c::Login,
}

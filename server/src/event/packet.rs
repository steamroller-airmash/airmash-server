use crate::{network::ConnectionId, protocol::client as c};
use hecs::Entity;
use std::time::Instant;

/// A packet has been recieved from a connection that has been associated with
/// an entity.
///
/// This will happen for any packet except [`Login`] and [`Backup`] as those are
/// supposed to occur on newly-opened connections.
///
/// [`Login`]: crate::protocol::client::Login
/// [`Backup`]: crate::protocol::client::Backup
#[derive(Copy, Clone, Debug)]
pub struct PacketEvent<P> {
  pub conn: ConnectionId,
  pub entity: Entity,
  // The time at which the packet was received
  pub time: Instant,
  pub packet: P,
}

/// A login packet was received from a connection.
///
/// Note that this doesn't guarantee that the player will log in successfully.
/// If you want to listen for that use the [`PlayerJoin`] event.
///
/// [`PlayerJoin`]: crate::event::PlayerJoin
#[derive(Clone, Debug)]
pub struct LoginEvent {
  pub conn: ConnectionId,
  pub packet: c::Login,
}

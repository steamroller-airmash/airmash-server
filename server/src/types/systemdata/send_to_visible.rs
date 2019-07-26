use specs::prelude::*;

use crate::component::collision::PlayerGrid;
use crate::protocol::ServerPacket;
use crate::types::collision::HitCircle;
use crate::types::{AssociatedConnection, Config, ConnectionId, Position};

#[derive(SystemData)]
pub struct SendToVisible<'a> {
	conns: super::SendToAll<'a>,
	config: Read<'a, Config>,
	associated: ReadStorage<'a, AssociatedConnection>,
	entities: Entities<'a>,
	grid: Read<'a, PlayerGrid>,
}

impl<'a> SendToVisible<'a> {
	/// Get the player associated with this connection, or none
	/// if the connection is not associated with any players.
	///
	/// This function is mainly useful for packet handlers.
	///
	/// # Example
	/// Consider a handler for a command `spawn-upgrade`, the
	/// event it recieves is a `(ConnectionId, Command)` tuple.
	///
	/// ```
	/// # extern crate airmash_server;
	/// use airmash_server::component::event::CommandEvent;
	/// use airmash_server::types::systemdata::SendToAll;
	/// # use std::marker::PhantomData;
	/// # use std::borrow::Cow;
	/// # fn main() {}
	/// # struct Temp<'a> { x: PhantomData<Cow<'a, str>> }
	/// # impl<'a> Temp<'a> {
	///
	/// // Within the event handler implementaiton.
	/// fn on_event(&mut self, evt: &CommandEvent, conns: SendToAll<'a>) {
	/// 	let (connection, ref data) = *evt;
	///
	/// 	let player = conns.associated_player(connection);
	///
	/// 	// Do stuff with player and data here...
	/// }
	/// # }
	/// ```
	pub fn associated_player(&self, conn: ConnectionId) -> Option<Entity> {
		self.conns.associated_player(conn)
	}

	/// Send a packet to the given connection.
	///
	/// This method will take ownership of its arguments.
	/// If you don't want to clone the data every time,
	/// use [`send_to_ref()`][0] instead.
	///
	/// [0]: #method.send_to_ref
	pub fn send_to<I>(&self, conn: ConnectionId, msg: I)
	where
		I: Into<ServerPacket>,
	{
		self.conns.send_to(conn, msg);
	}

	/// Send a packet to the given connection.
	pub fn send_to_ref(&self, conn: ConnectionId, msg: &ServerPacket) {
		self.conns.send_to_ref(conn, msg);
	}

	/// Send a packet to the primary connection of a player.
	pub fn send_to_player<I>(&self, player: Entity, msg: I)
	where
		I: Into<ServerPacket>,
	{
		self.conns.send_to_player(player, msg)
	}

	/// Send a packet to the primary connection of all players.
	pub fn send_to_all<I>(&self, msg: I)
	where
		I: Into<ServerPacket>,
	{
		self.conns.send_to_all(msg)
	}

	/// Send a packet to all other players.
	pub fn send_to_others<I>(&self, player: Entity, msg: I)
	where
		I: Into<ServerPacket>,
	{
		self.conns.send_to_others(player, msg)
	}

	/// Send to to all players that could be visible
	pub fn send_to_visible<I>(&self, pos: Position, msg: I)
	where
		I: Into<ServerPacket>,
	{
		let ent = self.entities.entity(0);
		let msg = msg.into();

		self.grid
			.0
			.rough_collide(HitCircle {
				pos: pos,
				rad: self.config.view_radius,
				layer: 0,
				ent: ent,
			})
			.into_iter()
			.filter_map(|x| self.associated.get(x))
			.for_each(|associated| {
				self.send_to_ref(associated.0, &msg);
			});
	}
}

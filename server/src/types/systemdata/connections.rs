use specs::prelude::*;

use crate::component::collision::PlayerGrid;
use crate::component::flag::IsPlayer;
use crate::protocol::ServerPacket;
use crate::types::collision::HitCircle;
use crate::types::{
	AssociatedConnection, Config, ConnectionId, Connections as ConnData, Position, Team,
};

#[derive(SystemData)]
pub struct Connections<'a> {
	config: Read<'a, Config>,
	entities: Entities<'a>,
	grid: Read<'a, PlayerGrid>,
	conns: Read<'a, ConnData>,

	is_player: ReadStorage<'a, IsPlayer>,
	associated: ReadStorage<'a, AssociatedConnection>,
	team: ReadStorage<'a, Team>,
}

pub struct SendIter<'a, 'b: 'a, It>
where
	It: Iterator<Item = Entity>,
{
	conns: &'b Connections<'a>,
	iter: It,
}

impl<'a, 'b: 'a, It> SendIter<'a, 'b, It>
where
	It: Iterator<Item = Entity> + 'b,
{
	fn new(conns: &'b Connections<'a>, iter: It) -> Self {
		Self { conns, iter }
	}

	/// Only send the message to players on the given team
	pub fn on_team(self, team: Team) -> SendIter<'a, 'b, impl Iterator<Item = Entity> + 'b> {
		let conns = self.conns;
		let iter = self
			.iter
			.filter(move |&ent| conns.team.get(ent).map(|&x| x == team).unwrap_or(false));

		SendIter::new(conns, iter)
	}

	/// Only send the message to players not on the given team
	pub fn not_on_team(self, team: Team) -> SendIter<'a, 'b, impl Iterator<Item = Entity> + 'b> {
		let conns = self.conns;
		let iter = self
			.iter
			.filter(move |&ent| conns.team.get(ent).map(|&x| x != team).unwrap_or(false));

		SendIter::new(conns, iter)
	}

	/// Exclude the given player from those being sent messages
	pub fn except(self, player: Entity) -> SendIter<'a, 'b, impl Iterator<Item = Entity> + 'b> {
		let iter = self.iter.filter(move |&ent| ent != player);
		SendIter::new(self.conns, iter)
	}

	pub fn send_all<I>(self, msg: I)
	where
		I: Into<ServerPacket>,
	{
		self.send_all_ref(&msg.into());
	}

	pub fn send_all_ref(self, msg: &ServerPacket) {
		let conns = self.conns;
		let iter = self.iter.filter_map(move |ent| conns.associated.get(ent));

		for assoc in iter {
			self.conns.send_to_ref(assoc.0, msg);
		}
	}
}

impl<'a> Connections<'a> {
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

	/// Send a packet to the given connection
	pub fn send_to_ref(&self, conn: ConnectionId, msg: &ServerPacket) {
		self.conns.send_to_ref(conn, msg);
	}

	/// Send a packet to the primary connection for the player.
	pub fn send_to_player<I>(&self, player: Entity, msg: I)
	where
		I: Into<ServerPacket>,
	{
		self.send_to_player_ref(player, &msg.into());
	}

	/// Send a packet to the primary connection for the player.
	pub fn send_to_player_ref(&self, player: Entity, msg: &ServerPacket) {
		if let Some(conn) = self.associated.get(player) {
			self.send_to_ref(conn.0, msg);
		} else {
			warn!(
				"Tried to send message to player {:?} with no associated connection!",
				player
			);
		}
	}

	pub fn send_iter(&self) -> SendIter<impl Iterator<Item = Entity> + '_> {
		let iter = (&*self.entities, self.is_player.mask()).join().map(|x| x.0);
		SendIter::new(&self, iter)
	}

	pub fn send_visible(&self, pos: Position) -> SendIter<impl Iterator<Item = Entity> + '_> {
		let ent = self.entities.entity(0);
		let vals = self.grid.0.rough_collide(HitCircle {
			pos: pos,
			rad: self.config.view_radius,
			layer: 0,
			ent: ent,
		});

		SendIter::new(&self, vals.into_iter())
	}

	/// Send a packet to the primary connection for all players.
	pub fn send_to_all<I>(&self, msg: I)
	where
		I: Into<ServerPacket>,
	{
		self.send_to_all_ref(&msg.into());
	}

	/// Send a packet to the primary connection for all players.
	pub fn send_to_all_ref(&self, msg: &ServerPacket) {
		self.send_iter().send_all_ref(msg);
	}

	/// Send to all players except one.
	pub fn send_to_others<I>(&self, player: Entity, msg: I)
	where
		I: Into<ServerPacket>,
	{
		self.send_to_others_ref(player, &msg.into())
	}

	/// Send to all players except one.
	pub fn send_to_others_ref(&self, player: Entity, msg: &ServerPacket) {
		self.send_iter().except(player).send_all_ref(msg);
	}

	/// Send to all players that could see the given position.
	pub fn send_to_visible<I>(&self, pos: Position, msg: I)
	where
		I: Into<ServerPacket>,
	{
		self.send_visible(pos).send_all(msg);
	}

	/// Send to all players that could see the given position except one.
	pub fn send_to_visible_others<I>(&self, pos: Position, ent: Entity, msg: I)
	where
		I: Into<ServerPacket>,
	{
		self.send_visible(pos).except(ent).send_all(msg);
	}

	/// Send to all players on a team
	pub fn send_to_team<I>(&self, team: Team, msg: I)
	where
		I: Into<ServerPacket>,
	{
		self.send_iter().on_team(team).send_all(msg);
	}

	/// Send to all players on the given team that have
	/// the given position within their horizon.
	pub fn send_to_team_visible<I>(&self, pos: Position, team: Team, msg: I)
	where
		I: Into<ServerPacket>,
	{
		self.send_visible(pos).on_team(team).send_all(msg);
	}
}

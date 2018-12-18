use specs::prelude::*;

use component::collision::PlayerGrid;
use protocol::ServerPacket;
use types::collision::HitCircle;
use types::{AssociatedConnection, Config, ConnectionId, Position};

#[derive(SystemData)]
pub struct SendToVisible<'a> {
	pub conns: super::SendToAll<'a>,
	pub config: Read<'a, Config>,
	pub associated: ReadStorage<'a, AssociatedConnection>,
	pub entities: Entities<'a>,
	pub grid: Read<'a, PlayerGrid>,
}

impl<'a> SendToVisible<'a> {
	pub fn associated_player(&self, conn: ConnectionId) -> Option<Entity> {
		self.conns.associated_player(conn)
	}

	pub fn send_to<I>(&self, conn: ConnectionId, msg: I)
	where
		I: Into<ServerPacket>,
	{
		self.conns.send_to(conn, msg);
	}

	pub fn send_to_ref(&self, conn: ConnectionId, msg: &ServerPacket) {
		self.conns.send_to_ref(conn, msg);
	}

	pub fn send_to_player<I>(&self, player: Entity, msg: I)
	where
		I: Into<ServerPacket>,
	{
		self.conns.send_to_player(player, msg)
	}

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

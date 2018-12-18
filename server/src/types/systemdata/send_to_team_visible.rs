use specs::prelude::*;

use component::collision::PlayerGrid;
use protocol::ServerPacket;
use types::collision::HitCircle;
use types::{AssociatedConnection, Config, Connections, Position, Team};

#[derive(SystemData)]
pub struct SendToTeamVisible<'a> {
	pub conns: Read<'a, Connections>,
	pub config: Read<'a, Config>,
	pub associated: ReadStorage<'a, AssociatedConnection>,
	pub entities: Entities<'a>,
	pub grid: Read<'a, PlayerGrid>,
	pub team: ReadStorage<'a, Team>,
}

impl<'a> SendToTeamVisible<'a> {
	pub fn send_to_player<I>(&self, player: Entity, msg: I)
	where
		I: Into<ServerPacket>,
	{
		if let Some(conn) = self.associated.get(player) {
			self.conns.send_to(conn.0, msg);
		} else {
			warn!(
				"Tried to send message to player {:?} with no associated connection!",
				player
			);
		}
	}

	pub fn send_to_team<I>(&self, team: Team, msg: I)
	where
		I: Into<ServerPacket>,
	{
		let msg = msg.into();

		(&self.associated, &self.team)
			.join()
			.filter(|(_, &t)| t == team)
			.for_each(|(assoc, _)| {
				self.conns.send_to_ref(assoc.0, &msg);
			});
	}

	pub fn send_to_team_visible<I>(&self, pos: Position, team: Team, msg: I)
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
			.filter(|x| self.team.get(*x).map(|&x| x == team).unwrap_or(false))
			.filter_map(|x| self.associated.get(x))
			.for_each(|associated| {
				self.conns.send_to_ref(associated.0, &msg);
			});
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
				self.conns.send_to_ref(associated.0, &msg);
			});
	}
}

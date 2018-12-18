use specs::prelude::*;

use types::{Connections, AssociatedConnection, Team};
use protocol::ServerPacket;

#[derive(SystemData)]
pub struct SendToTeam<'a> {
	pub conns: Read<'a ,Connections>,
	pub associated: ReadStorage<'a, AssociatedConnection>,
	pub entities: Entities<'a>,
	pub team: ReadStorage<'a, Team>,
}

impl<'a> SendToTeam<'a> {
	pub fn send_to_player<I>(&self, player: Entity, msg: I)
	where
		I: Into<ServerPacket>
	{
		if let Some(conn) = self.associated.get(player) {
			self.conns.send_to(conn.0, msg);
		} else {
			warn!("Tried to send message to player {:?} with no associated connection!", player);
		}
	}

	pub fn send_to_team<I>(&self, team: Team, msg: I)
	where
		I: Into<ServerPacket>
	{
		let msg = msg.into();

		(
			&self.associated,
			&self.team
		).join()
			.filter(|(_, &t)| t == team)
			.for_each(|(assoc, _)| {
				self.conns.send_to_ref(assoc.0, &msg);
			});
	}
}


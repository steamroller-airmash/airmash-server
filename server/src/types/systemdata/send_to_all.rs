use specs::*;
use types::*;

use protocol::ServerPacket;

#[derive(SystemData)]
pub struct SendToAll<'a> {
	pub conns: Read<'a, Connections>,
	pub associated: ReadStorage<'a, AssociatedConnection>,
	pub entities: Entities<'a>,
}

impl<'a> SendToAll<'a> {
	pub fn associated_player(&self, conn: ConnectionId) -> Option<Entity> {
		self.conns.associated_player(conn)
	}

	pub fn send_to<I>(&self, conn: ConnectionId, msg: I)
	where
		I: Into<ServerPacket>,
	{
		self.conns.send_to(conn, msg);
	}

	pub fn send_to_all<I>(&self, msg: I)
	where
		I: Into<ServerPacket>,
	{
		let msg = msg.into();

		(&self.associated,).join().for_each(|(assoc,)| {
			self.conns.send_to_ref(assoc.0, &msg);
		});
	}

	pub fn send_to_others<I>(&self, player: Entity, msg: I)
	where
		I: Into<ServerPacket>,
	{
		let msg = msg.into();

		(&*self.entities, &self.associated)
			.join()
			.filter(|(ent, _)| *ent == player)
			.for_each(|(_, assoc)| {
				self.conns.send_to_ref(assoc.0, &msg);
			});
	}
}

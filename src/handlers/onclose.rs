use airmash_protocol::server::PlayerLeave;
use airmash_protocol::{to_bytes, ServerPacket};
use shrev::*;
use specs::*;
use types::*;
use websocket::OwnedMessage;

pub struct OnCloseHandler {
	reader: Option<ReaderId<ConnectionClose>>,
}

impl OnCloseHandler {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for OnCloseHandler {
	type SystemData = (
		Entities<'a>,
		Read<'a, EventChannel<ConnectionClose>>,
		Write<'a, Connections>,
	);

	fn setup(&mut self, res: &mut Resources) {
		self.reader = Some(
			res.fetch_mut::<EventChannel<ConnectionClose>>()
				.register_reader(),
		);

		Self::SystemData::setup(res);
	}

	fn run(&mut self, (entities, channel, mut connections): Self::SystemData) {
		if let Some(ref mut reader) = self.reader {
			for evt in channel.read(reader) {
				let (player, ty) = {
					let conn = connections.0.get(&evt.conn).unwrap_or_else(|| {
						error!(
							target: "server",
							"Attempted to close non-existent connection {:?}",
							evt.conn
						);
						panic!("Connection {:?} not found", evt.conn);
					});
					(conn.player, conn.ty)
				};

				if ty == ConnectionType::Primary {
					if let Some(ent) = player {
						connections.remove_player(ent);

						// Send out PlayerLeave message
						let player_leave = PlayerLeave {
							id: ent.id() as u16,
						};
						connections.send_to_all(OwnedMessage::Binary(
							to_bytes(&ServerPacket::PlayerLeave(player_leave)).unwrap(),
						));

						// Delete player entity
						entities.delete(ent).unwrap();

						// Log
						info!(
							target: "server",
							"Player {:?} left",
							ent
						);
					} else {
						connections.remove(evt.conn);
					}
				} else {
					connections.remove(evt.conn);
				}

				info!(
					target: "server",
					"{:?} closed",
					evt.conn
				);
			}
		}
	}
}

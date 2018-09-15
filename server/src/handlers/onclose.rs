use airmash_protocol::server::PlayerLeave;
use shrev::*;
use specs::*;
use types::*;

use component::channel::{OnClose, OnPlayerLeave};
use component::counter::PlayersGame;
use component::event::PlayerLeave as EvtPlayerLeave;
use dispatch::SystemInfo;
use handlers::OnOpenHandler;
use types::event::ConnectionClose;

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
		Read<'a, OnClose>,
		Write<'a, Connections>,
		Write<'a, PlayersGame>,
		Write<'a, OnPlayerLeave>,
	);

	fn setup(&mut self, res: &mut Resources) {
		self.reader = Some(
			res.fetch_mut::<EventChannel<ConnectionClose>>()
				.register_reader(),
		);

		Self::SystemData::setup(res);
	}

	fn run(
		&mut self,
		(entities, channel, mut connections, mut players, mut onleave): Self::SystemData,
	) {
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
						players.0 -= 1;

						// Send out PlayerLeave message
						let player_leave = PlayerLeave { id: ent.into() };
						connections.send_to_all(player_leave);

						onleave.single_write(EvtPlayerLeave(ent));
						// Delete player entity
						entities.delete(ent).unwrap();

						// Log
						info!("Player {:?} left", ent);
					} else {
						connections.remove(evt.conn);
					}
				} else {
					connections.remove(evt.conn);
				}
			}
		}
	}
}

impl SystemInfo for OnCloseHandler {
	type Dependencies = OnOpenHandler;

	fn new() -> Self {
		Self::new()
	}

	fn name() -> &'static str {
		concat!(module_path!(), line!())
	}
}

use crate::component::flag::IsPlayer;
use crate::component::time::LastKeyTime;
use crate::protocol::{server::Error, ErrorType};
use crate::task::TaskData;
use crate::types::{systemdata::Connections, Config};

use specs::prelude::*;

use std::time::Instant;

#[derive(SystemData)]
struct AfkData<'a> {
	entities: Entities<'a>,
	last_key: ReadStorage<'a, LastKeyTime>,
	is_player: ReadStorage<'a, IsPlayer>,

	conns: Connections<'a>,
}

/// If a player hasn't pressed a key in 30 minutes then
/// disconnect them for being AFK.
pub async fn afk_timeout(mut task: TaskData) {
	loop {
		let afk_timeout = task.read_resource::<Config, _, _>(|config| config.afk_timeout);

		task.sleep_for(afk_timeout).await;

		task.world(|world| {
			let AfkData {
				entities,
				last_key,
				is_player,
				conns,
			} = world.system_data();

			let now = Instant::now();
			let iter = (&*entities, &last_key, is_player.mask()).join();
			for (player, time, ..) in iter {
				if now - time.0 > afk_timeout {
					conns.send_to_player(
						player,
						Error {
							error: ErrorType::AfkTimeout,
						},
					);

					for conn in conns.associated_connections(player) {
						conns.close(conn);
					}
				}
			}
		});
	}
}

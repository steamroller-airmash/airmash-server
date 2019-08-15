use crate::component::flag::IsPlayer;
use crate::component::time::LastKeyTime;
use crate::task::TaskData;
use crate::types::systemdata::SendToPlayer;

use specs::{Entities, Join, ReadStorage};

use std::time::{Duration, Instant};

// Have an AFK timeout of 30 mins
const AFK_TIMEOUT: Duration = Duration::from_secs(60 * 30);

#[derive(SystemData)]
struct AfkData<'a> {
	entities: Entities<'a>,
	last_key: ReadStorage<'a, LastKeyTime>,
	is_player: ReadStorage<'a, IsPlayer>,

	conns: SendToPlayer<'a>,
}

/// If a player hasn't pressed a key in 30 minutes then
/// disconnect them for being AFK.
pub async fn afk_timeout(mut task: TaskData) {
	loop {
		task.sleep_for(AFK_TIMEOUT).await;

		task.world(|world| {
			let AfkData {
				entities,
				last_key,
				is_player,
				conns,
			} = world.system_data();

			let now = Instant::now();
			let iter = (&*entities, &last_key, is_player.mask()).join();
			for (ent, time, ..) in iter {
				if now - time.0 > AFK_TIMEOUT {
					todo!()
				}
			}
		});
	}
}

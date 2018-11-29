use specs::*;

use component::channel::*;
use component::event::*;
use component::flag::{IsDead, IsSpectating};
use consts::timer::*;

use utils::{EventHandler, EventHandlerTypeProvider};

use systems::TimerHandler;
use SystemInfo;

#[derive(Default)]
pub struct PlayerRespawnSystem;

#[derive(SystemData)]
pub struct PlayerRespawnSystemData<'a> {
	respawn_channel: Write<'a, OnPlayerRespawn>,
	entities: Entities<'a>,
	is_dead: WriteStorage<'a, IsDead>,
	is_spec: ReadStorage<'a, IsSpectating>,
}

impl EventHandlerTypeProvider for PlayerRespawnSystem {
	type Event = TimerEvent;
}

impl<'a> EventHandler<'a> for PlayerRespawnSystem {
	type SystemData = PlayerRespawnSystemData<'a>;

	fn on_event(&mut self, evt: &TimerEvent, data: &mut Self::SystemData) {
		if evt.ty != *RESPAWN_TIME {
			return;
		}

		let player = match evt.data {
			Some(ref dat) => match (*dat).downcast_ref::<Entity>() {
				Some(val) => *val,
				None => {
					error!("Unable to downcast TimerEvent data to Entity! Event will be skipped.");
					return;
				}
			},
			None => return,
		};

		if !data.entities.is_alive(player) {
			return;
		}

		data.is_dead.remove(player);

		if data.is_spec.get(player).is_some() {
			return;
		}

		data.respawn_channel.single_write(PlayerRespawn {
			player,
			prev_status: PlayerRespawnPrevStatus::Dead,
		});
	}
}

impl SystemInfo for PlayerRespawnSystem {
	type Dependencies = TimerHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

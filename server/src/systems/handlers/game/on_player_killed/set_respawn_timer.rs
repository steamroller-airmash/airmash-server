use specs::*;

use crate::types::*;
use crate::SystemInfo;

use std::time::Duration;

use crate::component::channel::*;
use crate::component::event::*;
use crate::consts::timer::RESPAWN_TIME;
use crate::systems::handlers::game::on_player_hit::AllPlayerHitSystems;
use crate::systems::missile::MissileHit;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct SetRespawnTimer;

#[derive(SystemData)]
pub struct SetRespawnTimerData<'a> {
	pub channel: Read<'a, OnPlayerKilled>,
	pub future: ReadExpect<'a, FutureDispatcher>,
}

impl EventHandlerTypeProvider for SetRespawnTimer {
	type Event = PlayerKilled;
}

impl<'a> EventHandler<'a> for SetRespawnTimer {
	type SystemData = SetRespawnTimerData<'a>;

	fn on_event(&mut self, evt: &PlayerKilled, data: &mut Self::SystemData) {
		let player = evt.player;

		data.future
			.run_delayed(Duration::from_secs(2), move |instant| {
				Some(TimerEvent {
					ty: *RESPAWN_TIME,
					instant,
					data: Some(Box::new(player)),
				})
			});
	}
}

impl SystemInfo for SetRespawnTimer {
	type Dependencies = (MissileHit, AllPlayerHitSystems);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Default::default()
	}
}

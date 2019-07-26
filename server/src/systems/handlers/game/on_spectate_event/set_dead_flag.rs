use specs::*;

use crate::component::event::PlayerSpectate;
use crate::component::flag::IsDead;
use crate::types::TaskSpawner;
use crate::utils::{EventHandler, EventHandlerTypeProvider};
use crate::SystemInfo;

#[derive(Default)]
pub struct SetDeadFlag;

#[derive(SystemData)]
pub struct SetDeadFlagData<'a> {
	is_dead: WriteStorage<'a, IsDead>,
	tasks: WriteExpect<'a, TaskSpawner>,
}

impl EventHandlerTypeProvider for SetDeadFlag {
	type Event = PlayerSpectate;
}

impl<'a> EventHandler<'a> for SetDeadFlag {
	type SystemData = SetDeadFlagData<'a>;

	fn on_event(&mut self, evt: &PlayerSpectate, data: &mut Self::SystemData) {
		if !evt.is_dead && !evt.is_spec {
			data.is_dead.insert(evt.player, IsDead).unwrap();

			let player = evt.player;
			let tdata = data.tasks.task_data();
			data.tasks
				.launch(crate::task::death_cooldown(tdata, player));
		}
	}
}

impl SystemInfo for SetDeadFlag {
	type Dependencies = super::KnownEventSources;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

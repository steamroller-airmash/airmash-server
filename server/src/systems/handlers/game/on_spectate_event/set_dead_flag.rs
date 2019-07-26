use specs::*;

use std::time::Duration;

use crate::component::event::{PlayerSpectate, TimerEvent};
use crate::component::flag::IsDead;
use crate::consts::timer::CLEAR_DEAD_FLAG;
use crate::types::FutureDispatcher;
use crate::utils::{EventHandler, EventHandlerTypeProvider};
use crate::SystemInfo;

#[derive(Default)]
pub struct SetDeadFlag;

#[derive(SystemData)]
pub struct SetDeadFlagData<'a> {
	is_dead: WriteStorage<'a, IsDead>,
	dispatch: ReadExpect<'a, FutureDispatcher>,
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

			data.dispatch
				.run_delayed(Duration::from_secs(2), move |inst| TimerEvent {
					instant: inst,
					ty: *CLEAR_DEAD_FLAG,
					data: Some(Box::new(player)),
				});
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

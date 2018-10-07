use specs::*;

use std::time::Duration;

use component::event::{PlayerSpectate, TimerEvent};
use component::flag::IsDead;
use consts::timer::CLEAR_DEAD_FLAG;
use types::FutureDispatcher;
use utils::{EventHandler, EventHandlerTypeProvider};
use SystemInfo;

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

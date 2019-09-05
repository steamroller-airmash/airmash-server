use crate::types::*;
use specs::prelude::*;

use crate::component::event::{PlayerThrottle, TimerEvent};
use crate::consts::timer::*;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

use std::time::Duration;

#[derive(Default)]
pub struct SetUnthrottleTimer;

#[derive(SystemDataCustom)]
pub struct SetUnthrottleTimerData<'a> {
	future: ReadExpect<'a, FutureDispatcher>,
}

impl EventHandlerTypeProvider for SetUnthrottleTimer {
	type Event = PlayerThrottle;
}

impl<'a> EventHandler<'a> for SetUnthrottleTimer {
	type SystemData = SetUnthrottleTimerData<'a>;

	fn on_event(&mut self, evt: &PlayerThrottle, data: &mut Self::SystemData) {
		let evt = evt.clone();
		data.future
			.run_delayed(Duration::from_secs(5), move |inst| {
				Some(TimerEvent {
					ty: *UNTHROTTLE_TIME,
					instant: inst,
					data: Some(Box::new(evt.clone())),
				})
			});
	}
}

system_info! {
	impl SystemInfo for SetUnthrottleTimer {
		type Dependencies = ();
	}
}

use specs::*;

use crate::systems::handlers::packet::LoginHandler;
use crate::SystemInfo;

use crate::component::event::*;
use crate::component::time::{LastStealthTime, StartTime};
use crate::utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct InitStealthTime;

#[derive(SystemData)]
pub struct InitStealthTimeData<'a> {
	start_time: Read<'a, StartTime>,

	last_stealth: WriteStorage<'a, LastStealthTime>,
}

impl EventHandlerTypeProvider for InitStealthTime {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for InitStealthTime {
	type SystemData = InitStealthTimeData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		data.last_stealth
			.insert(evt.id, LastStealthTime(data.start_time.0))
			.unwrap();
	}
}

impl SystemInfo for InitStealthTime {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

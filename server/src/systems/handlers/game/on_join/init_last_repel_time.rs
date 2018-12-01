use specs::*;

use SystemInfo;

use systems::handlers::packet::LoginHandler;

use component::event::*;
use component::time::*;
use utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct InitLastRepelTime;

#[derive(SystemData)]
pub struct InitLastRepelTimeData<'a> {
	this_frame: Read<'a, ThisFrame>,
	join_time: WriteStorage<'a, LastRepelTime>,
}

impl EventHandlerTypeProvider for InitLastRepelTime {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for InitLastRepelTime {
	type SystemData = InitLastRepelTimeData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		data.join_time
			.insert(evt.id, LastRepelTime(data.this_frame.0))
			.unwrap();
	}
}

impl SystemInfo for InitLastRepelTime {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

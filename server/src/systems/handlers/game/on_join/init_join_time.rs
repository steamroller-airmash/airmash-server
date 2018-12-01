use specs::*;

use SystemInfo;

use systems::handlers::packet::LoginHandler;

use component::event::*;
use component::time::*;
use utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct InitJoinTime;

#[derive(SystemData)]
pub struct InitJoinTimeData<'a> {
	pub this_frame: Read<'a, ThisFrame>,

	pub join_time: WriteStorage<'a, JoinTime>,
}

impl EventHandlerTypeProvider for InitJoinTime {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for InitJoinTime {
	type SystemData = InitJoinTimeData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		data.join_time
			.insert(evt.id, JoinTime(data.this_frame.0))
			.unwrap();
	}
}

impl SystemInfo for InitJoinTime {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

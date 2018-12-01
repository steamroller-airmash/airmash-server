use specs::*;

use types::*;

use SystemInfo;

use systems::handlers::packet::LoginHandler;

use component::event::*;
use utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct InitName;

#[derive(SystemData)]
pub struct InitNameData<'a> {
	names: WriteStorage<'a, Name>,
}

impl EventHandlerTypeProvider for InitName {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for InitName {
	type SystemData = InitNameData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		data.names.insert(evt.id, evt.name.clone()).unwrap();
	}
}

impl SystemInfo for InitName {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

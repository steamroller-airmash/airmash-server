use specs::*;

use crate::systems::handlers::packet::LoginHandler;
use crate::types::*;
use crate::SystemInfo;

use crate::component::event::*;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

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

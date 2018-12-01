use specs::*;

use types::*;

use SystemInfo;

use component::event::PlayerJoin;
use systems::handlers::packet::LoginHandler;
use utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct InitConnection;

#[derive(SystemData)]
pub struct InitConnectionData<'a> {
	pub conns: Write<'a, Connections>,
	pub associated: WriteStorage<'a, AssociatedConnection>,
}

impl EventHandlerTypeProvider for InitConnection {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for InitConnection {
	type SystemData = InitConnectionData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		data.conns
			.associate(evt.conn, evt.id, ConnectionType::Primary);
		data.associated
			.insert(evt.id, AssociatedConnection(evt.conn))
			.unwrap();
	}
}

impl SystemInfo for InitConnection {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

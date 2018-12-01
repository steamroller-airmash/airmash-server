use specs::*;
use types::*;

use SystemInfo;

use component::event::*;
use protocol::server::PlayerLevel;
use protocol::PlayerLevelType;
use utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct SendPlayerLevel;

#[derive(SystemData)]
pub struct SendPlayerLevelData<'a> {
	conns: Read<'a, Connections>,

	level: ReadStorage<'a, Level>,
}

impl EventHandlerTypeProvider for SendPlayerLevel {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for SendPlayerLevel {
	type SystemData = SendPlayerLevelData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		let packet = PlayerLevel {
			id: evt.id.into(),
			ty: PlayerLevelType::Login,
			level: *try_get!(evt.id, data.level),
		};

		data.conns.send_to_others(evt.id, packet);
	}
}

impl SystemInfo for SendPlayerLevel {
	type Dependencies = (super::InitTraits, super::SendLogin, super::InitConnection);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

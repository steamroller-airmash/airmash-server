use specs::*;
use types::*;

use component::channel::OnPlayerDespawn;
use component::event::{PlayerDespawn, PlayerDespawnType, PlayerLeave};

use utils::{EventHandler, EventHandlerTypeProvider};
use SystemInfo;

/// Create a despawn event when a player leaves
#[derive(Default)]
pub struct CreateDespawnEvent;

#[derive(SystemData)]
pub struct CreateDespawnEventData<'a> {
	channel: Write<'a, OnPlayerDespawn>,
	pos: ReadStorage<'a, Position>,
}

impl EventHandlerTypeProvider for CreateDespawnEvent {
	type Event = PlayerLeave;
}

impl<'a> EventHandler<'a> for CreateDespawnEvent {
	type SystemData = CreateDespawnEventData<'a>;

	fn on_event(&mut self, evt: &PlayerLeave, data: &mut Self::SystemData) {
		let &pos = data.pos.get(evt.0).unwrap();

		data.channel.single_write(PlayerDespawn {
			ty: PlayerDespawnType::Disconnect,
			player: evt.0,
			pos,
		})
	}
}

impl SystemInfo for CreateDespawnEvent {
	type Dependencies = super::KnownEventSources;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

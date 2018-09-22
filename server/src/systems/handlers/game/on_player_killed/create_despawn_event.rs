use specs::*;
use types::*;

use component::channel::OnPlayerDespawn;
use component::event::{PlayerDespawn, PlayerDespawnType, PlayerKilled};

use utils::{EventHandler, EventHandlerTypeProvider};
use SystemInfo;

/// Create a despawn event when a player dies
#[derive(Default)]
pub struct CreateDespawnEvent;

#[derive(SystemData)]
pub struct CreateDespawnEventData<'a> {
	channel: Write<'a, OnPlayerDespawn>,
	pos: ReadStorage<'a, Position>,
}

impl EventHandlerTypeProvider for CreateDespawnEvent {
	type Event = PlayerKilled;
}

impl<'a> EventHandler<'a> for CreateDespawnEvent {
	type SystemData = CreateDespawnEventData<'a>;

	fn on_event(&mut self, evt: &PlayerKilled, data: &mut Self::SystemData) {
		let &pos = data.pos.get(evt.player).unwrap();

		data.channel.single_write(PlayerDespawn {
			ty: PlayerDespawnType::Killed,
			player: evt.player,
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

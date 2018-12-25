use specs::*;
use types::systemdata::*;

use SystemInfo;

use utils::{EventHandler, EventHandlerTypeProvider};

use component::event::PowerupExpired;
use component::time::{LastUpdate, StartTime};

#[derive(Default)]
pub struct ForceUpdate;

#[derive(SystemData)]
pub struct ForceUpdateData<'a> {
	entities: Entities<'a>,
	is_alive: IsAlive<'a>,
	game_start: Read<'a, StartTime>,

	last_update: WriteStorage<'a, LastUpdate>,
}

impl EventHandlerTypeProvider for ForceUpdate {
	type Event = PowerupExpired;
}

impl<'a> EventHandler<'a> for ForceUpdate {
	type SystemData = ForceUpdateData<'a>;

	fn on_event(&mut self, evt: &Self::Event, data: &mut Self::SystemData) {
		if !data.entities.is_alive(evt.player) {
			return;
		}

		if !data.is_alive.get(evt.player) {
			return;
		}

		data.last_update
			.insert(evt.player, LastUpdate(data.game_start.0))
			.unwrap();
	}
}

impl SystemInfo for ForceUpdate {
	// This system has no dependencies, and it doesn't really matter
	// if it happens one frame, or the next.
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

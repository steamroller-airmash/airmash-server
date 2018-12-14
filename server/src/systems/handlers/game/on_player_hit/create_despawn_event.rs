use specs::*;

use component::channel::OnMissileDespawn;
use component::event::{MissileDespawn, MissileDespawnType, PlayerHit};
use types::Position;
use utils::*;
use SystemInfo;

#[derive(Default)]
pub struct CreateDespawnEvent;

#[derive(SystemData)]
pub struct CreateDespawnEventData<'a> {
	channel: Write<'a, OnMissileDespawn>,
	pos: ReadStorage<'a, Position>,
}

impl EventHandlerTypeProvider for CreateDespawnEvent {
	type Event = PlayerHit;
}

impl<'a> EventHandler<'a> for CreateDespawnEvent {
	type SystemData = CreateDespawnEventData<'a>;

	fn on_event(&mut self, evt: &PlayerHit, data: &mut Self::SystemData) {
		let pos = *try_get!(evt.missile, data.pos);

		data.channel.single_write(MissileDespawn {
			missile: evt.missile,
			pos: pos,
			ty: MissileDespawnType::HitPlayer,
		});
	}
}

impl SystemInfo for CreateDespawnEvent {
	type Dependencies = (super::KnownEventSources,);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

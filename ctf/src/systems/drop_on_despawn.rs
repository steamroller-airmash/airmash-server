use specs::*;

use crate::component::*;

use crate::server::component::event::*;
use crate::server::systems::handlers::game::on_player_despawn::KnownEventSources;
use crate::server::utils::{EventHandler, EventHandlerTypeProvider};
use crate::server::*;

/// Drop a carried flag when a player despawns.
#[derive(Default)]
pub struct DropOnDespawn;

#[derive(SystemData)]
pub struct DropOnDespawnData<'a> {
	channel: Write<'a, OnFlag>,
	entities: Entities<'a>,

	carrier: WriteStorage<'a, FlagCarrier>,
	is_flag: ReadStorage<'a, IsFlag>,
}

impl EventHandlerTypeProvider for DropOnDespawn {
	type Event = PlayerDespawn;
}

impl<'a> EventHandler<'a> for DropOnDespawn {
	type SystemData = DropOnDespawnData<'a>;

	fn on_event(&mut self, evt: &PlayerDespawn, data: &mut Self::SystemData) {
		let player = evt.player;
		let ref mut channel = data.channel;

		(&*data.entities, &mut data.carrier, &data.is_flag)
			.join()
			.filter(|(_, carrier, ..)| carrier.0.is_some())
			.filter(|(_, carrier, ..)| carrier.0.unwrap() == player)
			.for_each(|(ent, carrier, ..)| {
				channel.single_write(FlagEvent {
					ty: FlagEventType::Drop,
					player: Some(player),
					flag: ent,
				});

				carrier.0 = None;
			});
	}
}

impl SystemInfo for DropOnDespawn {
	type Dependencies = (KnownEventSources, super::PickupFlagSystem);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

use specs::*;

use airmash_server::utils::{EventHandler, EventHandlerTypeProvider};
use airmash_server::component::event::PlayerRespawn;
use airmash_server::systems::handlers::game::on_player_respawn::KnownEventSources;

use crate::component::*;

/// If a player happens to have the flag when respawning
/// then reset the flag back to it's home position.
#[derive(Default)]
pub struct DropFlag;

#[derive(SystemData)]
pub struct DropFlagData<'a> {
	channel: Write<'a, OnFlag>,
	carrier: WriteStorage<'a, FlagCarrier>,
	flag: ReadStorage<'a, IsFlag>,
	entities: Entities<'a>, 
}

impl EventHandlerTypeProvider for DropFlag {
	type Event = PlayerRespawn;
}

impl<'a> EventHandler<'a> for DropFlag {
	type SystemData = DropFlagData<'a>;

	fn on_event(&mut self, evt: &PlayerRespawn, data: &mut DropFlagData) {
		let ref mut channel = data.channel;

		(
			&mut data.carrier,
			&*data.entities,
			&data.flag
		).join()
			.filter_map(|(carrier, ent, ..)| carrier.0.map(move |x| (x, ent)))
			.for_each(|(carrier, ent)| {
				if evt.player == carrier {
					channel.single_write(FlagEvent {
						ty: FlagEventType::Return,
						player: None,
						flag: ent
					});
				}
			})	
	}
}

system_info! {
	impl SystemInfo for DropFlag {
		type Dependencies = KnownEventSources;
	}
}

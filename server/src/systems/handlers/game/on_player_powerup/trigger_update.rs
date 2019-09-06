use specs::prelude::*;

use crate::component::event::PlayerPowerup;
use crate::component::flag::ForcePlayerUpdate;

/// Forces a `PlayerUpdate` packet to be sent out when
/// a player is given a powerup.
#[event_handler(name=TriggerUpdate)]
fn trigger_update<'a>(
	evt: &PlayerPowerup,
	entities: &Entities<'a>,
	force_update: &mut WriteStorage<'a, ForcePlayerUpdate>,
) {
	if !entities.is_alive(evt.player) {
		return;
	}

	force_update.insert(evt.player, ForcePlayerUpdate).unwrap();
}

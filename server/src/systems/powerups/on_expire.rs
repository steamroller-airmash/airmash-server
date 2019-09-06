//! Event handlers for when a powerup on a player
//! expires.

use crate::{
	component::{player::ForcePlayerUpdate, powerup::PowerupExpire},
	types::systemdata::IsAlive,
};
use specs::prelude::*;

#[event_handler(name = ForceUpdate)]
fn force_update<'a>(
	evt: &PowerupExpire,

	entities: &Entities<'a>,
	is_alive: &IsAlive<'a>,
	force_update: &mut WriteStorage<'a, ForcePlayerUpdate>,
) {
	if !entities.is_alive(evt.player) || !is_alive.get(evt.player) {
		return;
	}

	force_update.insert(evt.player, ForcePlayerUpdate).unwrap();
}

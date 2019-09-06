mod send_player_powerup;
mod trigger_update;

pub use self::send_player_powerup::SendPlayerPowerup;
pub use self::trigger_update::TriggerUpdate;

pub type AllPlayerPowerupSystems = (SendPlayerPowerup, TriggerUpdate, GivePowerup);
pub type KnownEventSources = crate::utils::EventSources<crate::component::event::PlayerPowerup>;

use crate::{component::player::PlayerPowerup, types::Powerups};
use specs::prelude::*;

#[event_handler(name = GivePowerup)]
fn give_powerup<'a>(
	evt: &PlayerPowerup,
	entities: &Entities<'a>,
	powerups: &mut WriteStorage<'a, Powerups>,
) {
	use std::time::Instant;

	if !entities.is_alive(evt.player) {
		return;
	}

	// Add powerup information to the player
	powerups
		.insert(
			evt.player,
			Powerups {
				end_time: Instant::now() + evt.duration,
				ty: evt.ty,
			},
		)
		.unwrap();
}

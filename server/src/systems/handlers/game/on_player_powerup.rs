pub type AllPlayerPowerupSystems = (SendPlayerPowerup, TriggerUpdate, GivePowerup);
pub type KnownEventSources = crate::utils::EventSources<crate::component::event::PlayerPowerup>;

use crate::{
	component::player::{ForcePlayerUpdate, PlayerPowerup},
	protocol::server::PlayerPowerup as ServerPlayerPowerup,
	types::{systemdata::Connections, Powerups},
};

use specs::prelude::*;

/// Fill out the [`Powerups`][0] structure for the player.
///
/// [0]: crate::types::Powerups
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

type SendPacketDeps = (crate::systems::handlers::game::on_player_respawn::SendPlayerRespawn);

/// Add the initial 2s shield when a player joins
/// and send that packet to all visible players.
#[event_handler(name=SendPlayerPowerup, deps=SendPacketDeps)]
fn send_packet<'a>(evt: &PlayerPowerup, conns: &Connections<'a>) {
	let duration = evt.duration.as_secs() * 1000 + evt.duration.subsec_millis() as u64;

	conns.send_to_player(
		evt.player,
		ServerPlayerPowerup {
			duration: duration as u32,
			ty: evt.ty,
		},
	);
}

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

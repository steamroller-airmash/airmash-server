use specs::prelude::*;
use std::time::Instant;

use crate::component::{player::IsPlayer, powerup::*};
use crate::protocol::PowerupType;
use crate::types::collision::Collision;
use crate::types::{Config, Mob, Position, PowerupSpawnPoints, Powerups};

mod inner {
	use super::*;

	#[derive(Copy, Clone, Debug, Default, Component)]
	#[storage(NullStorage)]
	pub struct AlreadyPickedUp;
}
use self::inner::*;

/// Things that need to happen once a player-powerup collision
/// has occurred are as follows:
///  - Give the powerup to the player. This overrides any other
///    existing powerup that the player has.
///  - Add a PowerupAlreadyPickedUp marker to the powerup entity
///    which should be checked to prevent duplicate collisions.
///  - Create a PowerupDespawn event which will take care of actually
///    cleaning up the powerup entity.
///  - Reset statically spawned powerups if this one happens to be
///    one of them.
#[event_handler(name=HandleCollision)]
fn handle_collision<'a>(
	evt: &PlayerPowerupCollision,
	entities: &Entities<'a>,
	config: &Read<'a, Config>,
	spawn_points: &mut Write<'a, PowerupSpawnPoints>,

	is_player: &ReadStorage<'a, IsPlayer>,
	pos: &ReadStorage<'a, Position>,
	mob: &ReadStorage<'a, Mob>,
	taken: &mut WriteStorage<'a, AlreadyPickedUp>,
	powerups: &mut WriteStorage<'a, Powerups>,

	despawn_channel: &mut Write<'a, OnPowerupDespawn>,
	powerup_channel: &mut Write<'a, OnPlayerPowerup>,
) {
	let Collision(c1, c2) = evt.0;

	// Filter out invalid collisions
	if !entities.is_alive(c1.ent) || entities.is_alive(c2.ent) {
		return;
	}

	let (player, powerup) = match is_player.get(c1.ent) {
		Some(_) => (c1, c2),
		None => (c2, c1),
	};

	// There may be duplicate collision events for the same event,
	// this filters them out.
	let prev = taken.insert(powerup.ent, AlreadyPickedUp).unwrap();
	if let Some(_) = prev {
		return;
	}

	// Get the proper duration and type of the powerup
	let (duration, ty) = match *try_get!(powerup.ent, mob) {
		Mob::Shield => (config.shield_duration, PowerupType::Shield),
		Mob::Inferno => (config.inferno_duration, PowerupType::Inferno),
		_ => return,
	};

	// Add powerup information to the player
	powerups
		.insert(
			player.ent,
			Powerups {
				end_time: Instant::now() + duration,
				ty,
			},
		)
		.unwrap();

	// Handle static powerup spawn points
	let psps = spawn_points
		.0
		.iter_mut()
		.filter(|p| p.powerup_entity.map(|e| e == powerup.ent).unwrap_or(false));

	for p in psps {
		p.powerup_entity = None;
		p.next_respawn_time = Some(Instant::now() + p.respawn_delay);
	}

	// Send out events
	despawn_channel.single_write(PowerupDespawn {
		mob: powerup.ent,
		ty: *try_get!(powerup.ent, mob),
		pos: *try_get!(powerup.ent, pos),
		player: Some(player.ent),
	});

	powerup_channel.single_write(PlayerPowerup {
		player: player.ent,
		duration,
		ty,
	});
}

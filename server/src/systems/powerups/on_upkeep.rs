use specs::prelude::*;

use crate::component::{powerup::*, time::ThisFrame};
use crate::types::{systemdata::IsAlive, Mob, Position, PowerupSpawnPoints, Powerups};

pub use super::spawn_random_powerup::SpawnRandomPowerup;

/// Spawn powerups at fixed locations on reliable
/// schedules.
#[system(name=SpawnFixedPowerup)]
fn spawn_fixed_powerup<'a>(
	entities: Entities<'a>,
	mut mob: WriteStorage<'a, Mob>,
	mut pos: WriteStorage<'a, Position>,
	mut is_powerup: WriteStorage<'a, IsPowerup>,
	mut spawn_points: Write<'a, PowerupSpawnPoints>,
	mut channel: Write<'a, OnPowerupSpawn>,
	this_frame: Read<'a, ThisFrame>,
) {
	let psps = spawn_points
		.0
		.iter_mut()
		.filter(|p| p.powerup_entity.is_none())
		.filter(|p| {
			p.next_respawn_time
				.map(|x| x <= this_frame.0)
				.unwrap_or(false)
		});

	for p in psps {
		let mob = entities
			.build_entity()
			.with(p.pos, &mut pos)
			.with(p.powerup_type, &mut mob)
			.with(IsPowerup, &mut is_powerup)
			.build();

		p.powerup_entity = Some(mob);

		channel.single_write(PowerupSpawn {
			mob,
			pos: p.pos,
			despawn: None,
			ty: p.powerup_type,
		});
	}
}

#[system(name = CheckExpired)]
fn check_expired<'a>(
	entities: Entities<'a>,
	powerups: ReadStorage<'a, Powerups>,
	mut channel: Write<'a, OnPowerupExpire>,
	is_alive: IsAlive<'a>,
	this_frame: Read<'a, ThisFrame>,
	lazy: Read<'a, LazyUpdate>,
) {
	let iter = (&*entities, &powerups, is_alive.mask())
		.join()
		.filter(|(_, powerup, ..)| powerup.end_time < this_frame.0);

	for (ent, powerup, ..) in iter {
		channel.single_write(PowerupExpire {
			player: ent,
			ty: powerup.ty,
		});

		lazy.remove::<Powerups>(ent);
	}
}

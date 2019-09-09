use specs::prelude::*;

use crate::component::{
	powerup::*,
	time::{MobDespawnTime, ThisFrame},
};
use crate::consts::config::{MAP_SIZE, POWERUP_RADIUS};
use crate::types::{
	collision::Terrain, systemdata::IsAlive, Mob, Position, PowerupSpawnPoints, Powerups, Vector2,
};

use rand::{random, Open01};
use std::time::{Duration, Instant};

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

lazy_static! {
	static ref TERRAIN: Terrain = Terrain::default();
}

// Chance that a powerup will spawn on the map each frame.
// TODO: Move these into Config
const SPAWN_CHANCE: f32 = 0.005;
const POWERUP_LIFETIME: u64 = 60;

#[system(name = SpawnRandomPowerup)]
fn spawn_random_powerup<'a>(
	entities: Entities<'a>,

	mut mobs: WriteStorage<'a, Mob>,
	mut positions: WriteStorage<'a, Position>,
	mut is_powerup: WriteStorage<'a, IsPowerup>,
	mut despawn_time: WriteStorage<'a, MobDespawnTime>,

	mut channel: Write<'a, OnPowerupSpawn>,
) {
	if random::<Open01<f32>>().0 > SPAWN_CHANCE {
		return;
	}

	let powerup_type = if random::<Open01<f32>>().0 > 0.5 {
		Mob::Shield
	} else {
		Mob::Inferno
	};

	let mut pos = None;

	for _ in 0..10 {
		let coords = Vector2::<f32>::new(random::<Open01<f32>>().0, random::<Open01<f32>>().0);
		let trypos = coords * MAP_SIZE - MAP_SIZE * 0.5;

		if !TERRAIN.buckets.does_collide(trypos, POWERUP_RADIUS) {
			pos = Some(trypos);
			break;
		}
	}

	let pos = match pos {
		Some(x) => x,
		None => return,
	};

	let despawn = Instant::now() + Duration::from_secs(POWERUP_LIFETIME);

	let mob = entities
		.build_entity()
		.with(pos, &mut positions)
		.with(powerup_type, &mut mobs)
		.with(MobDespawnTime(despawn), &mut despawn_time)
		.with(IsPowerup, &mut is_powerup)
		.build();

	channel.single_write(PowerupSpawn {
		mob,
		pos,
		despawn: Some(despawn),
		ty: powerup_type,
	});
}

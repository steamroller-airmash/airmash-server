use specs::*;

use component::channel::OnPowerupSpawn;
use component::event::PowerupSpawnEvent;
use component::flag::IsPowerup;
use component::time::MobDespawnTime;
use consts::config::{MAP_SIZE, POWERUP_RADIUS};
use types::collision::Terrain;
use types::*;

use rand::{random, Open01};
use std::time::{Duration, Instant};

// Chance that a shield will spawn on the map each frame.
const SPAWN_CHANCE: f32 = 0.01;
const SHIELD_LIFETIME: u64 = 60;

#[derive(Default)]
pub struct SpawnShield {
	terrain: Terrain,
}

#[derive(SystemData)]
pub struct SpawnShieldData<'a> {
	entities: Entities<'a>,
	mob: WriteStorage<'a, Mob>,
	despawn_time: WriteStorage<'a, MobDespawnTime>,
	pos: WriteStorage<'a, Position>,
	is_powerup: WriteStorage<'a, IsPowerup>,

	channel: Write<'a, OnPowerupSpawn>,
}

impl<'a> System<'a> for SpawnShield {
	type SystemData = SpawnShieldData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		if random::<Open01<f32>>().0 > SPAWN_CHANCE {
			return;
		}

		let coords = Vector2::<f32>::new(random::<Open01<f32>>().0, random::<Open01<f32>>().0);
		let pos = coords * MAP_SIZE - MAP_SIZE * 0.5;

		if self.terrain.buckets.does_collide(pos, POWERUP_RADIUS) {
			// Don't spawn powerups that collide with the terrain
			return;
		}

		let despawn = Instant::now() + Duration::from_secs(SHIELD_LIFETIME);

		let mob = data
			.entities
			.build_entity()
			.with(pos, &mut data.pos)
			.with(Mob::Shield, &mut data.mob)
			.with(MobDespawnTime(despawn), &mut data.despawn_time)
			.with(IsPowerup, &mut data.is_powerup)
			.build();

		data.channel.single_write(PowerupSpawnEvent {
			mob,
			pos,
			despawn,
			ty: Mob::Shield,
		});
	}
}

system_info! {
	impl SystemInfo for SpawnShield {
		type Dependencies = ();
	}
}

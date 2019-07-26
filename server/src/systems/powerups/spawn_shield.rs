use specs::world::EntitiesRes;
use specs::*;

use crate::component::channel::OnPowerupSpawn;
use crate::component::event::PowerupSpawnEvent;
use crate::component::flag::IsPowerup;
use crate::component::time::MobDespawnTime;
use crate::consts::config::{MAP_SIZE, POWERUP_RADIUS};
use crate::types::collision::Terrain;
use crate::types::*;

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

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.terrain = Terrain::from_default(&*res.fetch::<EntitiesRes>());
	}

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

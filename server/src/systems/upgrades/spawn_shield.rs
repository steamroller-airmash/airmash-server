use specs::*;

use component::channel::OnPowerupSpawn;
use component::event::PowerupSpawnEvent;
use component::time::MobDespawnTime;
use consts::config::MAP_SIZE;
use types::*;
use SystemInfo;

use std::time::{Duration, Instant};

use rand::{self, Open01};

// Chance that a shield will spawn on the map each frame.
const SPAWN_CHANCE: f32 = 0.05;
const SHIELD_LIFETIME: u64 = 60;

#[derive(Default)]
pub struct SpawnShield;

#[derive(SystemData)]
pub struct SpawnShieldData<'a> {
	entities: Entities<'a>,
	mob: WriteStorage<'a, Mob>,
	despawn_time: WriteStorage<'a, MobDespawnTime>,
	pos: WriteStorage<'a, Position>,

	channel: Write<'a, OnPowerupSpawn>,
}

impl<'a> System<'a> for SpawnShield {
	type SystemData = SpawnShieldData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		if rand::random::<Open01<f32>>().0 > SPAWN_CHANCE {
			return;
		}

		let x = rand::random::<Open01<f32>>().0 * MAP_SIZE.x - MAP_SIZE.x * 0.5;
		let y = rand::random::<Open01<f32>>().0 * MAP_SIZE.y - MAP_SIZE.y * 0.5;

		let despawn = Instant::now() + Duration::from_secs(SHIELD_LIFETIME);

		let mob = data
			.entities
			.build_entity()
			.with(Position::new(x, y), &mut data.pos)
			.with(Mob::Shield, &mut data.mob)
			.with(MobDespawnTime(despawn), &mut data.despawn_time)
			.build();

		data.channel.single_write(PowerupSpawnEvent {
			mob,
			pos: Position::new(x, y),
			despawn,
			ty: Mob::Shield,
		});
	}
}

impl SystemInfo for SpawnShield {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

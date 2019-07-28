use specs::*;

use component::channel::OnPowerupSpawn;
use component::event::PowerupSpawnEvent;
use component::flag::IsPowerup;
use component::time::ThisFrame;
use types::*;

#[derive(Default)]
pub struct SpawnFixedPowerup {}

#[derive(SystemData)]
pub struct SpawnFixedPowerupData<'a> {
	entities: Entities<'a>,

	mob: WriteStorage<'a, Mob>,
	pos: WriteStorage<'a, Position>,
	is_powerup: WriteStorage<'a, IsPowerup>,

	powerup_spawn_points: Write<'a, PowerupSpawnPoints>,

	channel: Write<'a, OnPowerupSpawn>,
	this_frame: Read<'a, ThisFrame>,
}

impl<'a> System<'a> for SpawnFixedPowerup {
	type SystemData = SpawnFixedPowerupData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);
	}

	fn run(&mut self, mut data: Self::SystemData) {
		let this_frame = *data.this_frame;
		let psps = data
			.powerup_spawn_points
			.0
			.iter_mut()
			.filter(|p| p.powerup_entity.is_none())
			.filter(|p| {
				p.next_respawn_time.is_none() || p.next_respawn_time.unwrap() <= this_frame.0
			});

		for p in psps {
			let mob = data
				.entities
				.build_entity()
				.with(p.pos, &mut data.pos)
				.with(p.powerup_type, &mut data.mob)
				.with(IsPowerup, &mut data.is_powerup)
				.build();

			p.powerup_entity = Some(mob);

			data.channel.single_write(PowerupSpawnEvent {
				mob,
				pos: p.pos,
				despawn: None,
				ty: p.powerup_type,
			});
		}
	}
}

system_info! {
	impl SystemInfo for SpawnFixedPowerup {
		type Dependencies = ();
	}
}

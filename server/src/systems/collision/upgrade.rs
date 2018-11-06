use fnv::FnvHashSet;
use specs::prelude::*;

use Mob;

use types::collision::*;
use types::*;

use component::channel::*;
use component::event::PlayerUpgradeCollision;
use component::flag::*;
use component::collision::PlaneGrid;

pub struct PlayerUpgradeCollisionSystem;

#[derive(SystemData)]
pub struct PlayerUpgradeCollisionSystemData<'a> {
	pub channel: Write<'a, OnPlayerUpgradeCollision>,
	pub ent: Entities<'a>,
	pub grid: Read<'a, PlaneGrid>,

	pub pos: ReadStorage<'a, Position>,
	pub team: ReadStorage<'a, Team>,

	pub mob: ReadStorage<'a, Mob>,
	pub missile_flag: ReadStorage<'a, IsMissile>,
}

impl PlayerUpgradeCollisionSystem {
	pub fn new() -> Self {
		Self {}
	}
}

impl<'a> System<'a> for PlayerUpgradeCollisionSystem {
	type SystemData = PlayerUpgradeCollisionSystemData<'a>;

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			mut channel,
			ent,
			grid,

			pos,
			team,

			mob,
			missile_flag,
		} = data;

		let grid = &grid.0;

		let collisions = (&*ent, &pos, &team, &mob, !missile_flag.mask())
			.par_join()
			.filter(|(_, _, _, &mob, ..)| mob == Mob::Upgrade)
			.map(|(ent, pos, team, mob, ..)| {
				let it = COLLIDERS[mob].iter()
					.map(|(offset, rad)| {
						HitCircle {
							pos: *pos + *offset,
							rad: *rad,
							layer: team.0,
							ent: ent,
						}
					});

				grid.collide(it)
			})
			.flatten()
			.map(PlayerUpgradeCollision)
			.collect::<FnvHashSet<_>>();

		channel.iter_write(collisions.into_iter());
	}
}

use dispatch::SystemInfo;
use systems::PositionUpdate;
use super::GenPlaneGrid;

impl SystemInfo for PlayerUpgradeCollisionSystem {
	type Dependencies = (PositionUpdate, GenPlaneGrid);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

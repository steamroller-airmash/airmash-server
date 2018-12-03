use specs::*;

use types::collision::*;
use types::*;
use SystemInfo;

use component::collision::MissileGrid;
use component::flag::IsMissile;

#[derive(Default)]
pub struct GenMissileGrid;

#[derive(SystemData)]
pub struct GenMissileGridData<'a> {
	grid: Write<'a, MissileGrid>,

	entities: Entities<'a>,
	pos: ReadStorage<'a, Position>,
	team: ReadStorage<'a, Team>,
	is_missile: ReadStorage<'a, IsMissile>,
}

impl<'a> System<'a> for GenMissileGrid {
	type SystemData = GenMissileGridData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let it = (
			&*data.entities,
			&data.pos,
			&data.team,
			data.is_missile.mask(),
		)
			.join()
			.map(|(ent, pos, team, ..)| HitCircle {
				pos: *pos,
				rad: Distance::new(0.0),
				ent: ent,
				layer: team.0,
			});

		data.grid.0.rebuild_from(it);
	}
}

use systems::PositionUpdate;

impl SystemInfo for GenMissileGrid {
	type Dependencies = PositionUpdate;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

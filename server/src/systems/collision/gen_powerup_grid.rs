use specs::*;

use types::collision::*;
use types::*;
use consts::config::POWERUP_RADIUS;

use component::collision::PowerupGrid;
use component::flag::IsPowerup;

#[derive(Default)]
pub struct GenPowerupGrid;

#[derive(SystemData)]
pub struct GenPowerupGridData<'a> {
	grid: Write<'a, PowerupGrid>,

	entities: Entities<'a>,
	pos: ReadStorage<'a, Position>,
	is_powerup: ReadStorage<'a, IsPowerup>,
}

impl<'a> System<'a> for GenPowerupGrid {
	type SystemData = GenPowerupGridData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let it = (&*data.entities, &data.pos, data.is_powerup.mask())
			.join()
			.map(|(ent, pos, ..)| HitCircle {
				pos: *pos,
				rad: Distance::new(POWERUP_RADIUS),
				ent: ent,
				layer: 0,
			});

		data.grid.rebuild_from(it);
	}
}

use systems::PositionUpdate;

system_info! {
	impl SystemInfo for GenPowerupGrid {
		type Dependencies = PositionUpdate;
	}
}

use specs::*;

use types::collision::*;
use types::*;
use SystemInfo;

use component::collision::PlayerGrid;
use component::flag::IsPlayer;

#[derive(Default)]
pub struct GenPlayerGrid;

#[derive(SystemData)]
pub struct GenPlayerGridData<'a> {
	grid: Write<'a, PlayerGrid>,

	entities: Entities<'a>,
	pos: ReadStorage<'a, Position>,
	team: ReadStorage<'a, Team>,
	is_player: ReadStorage<'a, IsPlayer>,
}

impl<'a> System<'a> for GenPlayerGrid {
	type SystemData = GenPlayerGridData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let it = (
			&*data.entities,
			&data.pos,
			&data.team,
			data.is_player.mask(),
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

impl SystemInfo for GenPlayerGrid {
	type Dependencies = PositionUpdate;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

use specs::*;

use SystemInfo;

use types::systemdata::IsAlive;
use types::Powerups;

use component::channel::OnPowerupExpired;
use component::event::PowerupExpired;
use component::time::ThisFrame;

use systems::handlers::game::on_player_powerup::SetPowerupLifetime;

#[derive(Default)]
pub struct CheckExpired;

#[derive(SystemData)]
pub struct CheckExpiredData<'a> {
	pub entities: Entities<'a>,
	pub powerups: WriteStorage<'a, Powerups>,
	pub channel: Write<'a, OnPowerupExpired>,
	pub is_alive: IsAlive<'a>,
	pub this_frame: Read<'a, ThisFrame>,
}

impl<'a> System<'a> for CheckExpired {
	type SystemData = CheckExpiredData<'a>;

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			entities,
			mut powerups,
			mut channel,
			is_alive,
			this_frame,
		} = data;

		let mut ents = vec![];

		(&*entities, &powerups, is_alive.mask())
			.join()
			.filter(|(_, powerup, ..)| powerup.end_time < this_frame.0)
			.map(|(ent, powerup, ..)| {
				ents.push(ent);

				PowerupExpired {
					player: ent,
					ty: powerup.ty,
				}
			})
			.for_each(|evt| channel.single_write(evt));

		for ent in ents {
			powerups.remove(ent).unwrap();
		}
	}
}

impl SystemInfo for CheckExpired {
	type Dependencies = (SetPowerupLifetime);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

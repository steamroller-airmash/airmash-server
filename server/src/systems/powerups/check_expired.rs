use specs::*;

use SystemInfo;

use types::systemdata::IsAlive;
use types::Powerups;

use component::channel::OnPowerupExpired;
use component::event::PowerupExpired;
use component::time::ThisFrame;

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

		let events = (&*entities, &mut powerups, is_alive.mask())
			.join()
			.filter(|(_, powerup, ..)| powerup.details.is_some())
			.filter(|(_, powerup, ..)| powerup.details.unwrap().end_time > this_frame.0)
			.map(|(ent, powerup, ..)| {
				let inner = powerup.details.unwrap();
				powerup.details = None;

				PowerupExpired {
					player: ent,
					ty: inner.ty,
				}
			});

		for evt in events {
			channel.single_write(evt);
		}
	}
}

impl SystemInfo for CheckExpired {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

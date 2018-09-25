use specs::*;
use types::systemdata::*;
use types::*;

use systems::handlers::packet::KeyHandler;
use systems::specials::config::*;

use component::channel::OnPlayerStealth;
use component::event::PlayerStealth;
use component::flag::*;
use component::time::{LastStealthTime, ThisFrame};

use protocol::PlaneType;
use SystemInfo;

pub struct SetStealth;

#[derive(SystemData)]
pub struct SetStealthData<'a> {
	pub config: Read<'a, Config>,
	pub entities: Entities<'a>,
	pub this_frame: Read<'a, ThisFrame>,
	pub channel: Write<'a, OnPlayerStealth>,

	pub plane: ReadStorage<'a, Plane>,
	pub keystate: WriteStorage<'a, KeyState>,
	pub energy: WriteStorage<'a, Energy>,
	pub last_stealth: WriteStorage<'a, LastStealthTime>,
	pub is_alive: IsAlive<'a>,
	pub is_player: ReadStorage<'a, IsPlayer>,
}

impl<'a> System<'a> for SetStealth {
	type SystemData = SetStealthData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let mut flips = vec![];
		let this_frame = *data.this_frame;

		(
			&data.plane,
			&mut data.energy,
			&data.keystate,
			&mut data.last_stealth,
			&*data.entities,
			data.is_alive.mask(),
			data.is_player.mask(),
		)
			.join()
			.filter(|(plane, ..)| **plane == PlaneType::Prowler)
			.filter(|(_, _, _, last_stealth, ..)| {
				this_frame.0 - last_stealth.0 > *PROWLER_SPECIAL_DELAY
			}).filter(|(_, _, keystate, ..)| keystate.special)
			.filter(|(_, energy, ..)| **energy > *PROWLER_SPECIAL_ENERGY)
			.for_each(|(_, energy, keystate, last_stealth, ent, ..)| {
				flips.push(ent);

				if !keystate.stealthed {
					*energy -= *PROWLER_SPECIAL_ENERGY;
				}
				*last_stealth = LastStealthTime(this_frame.0);
			});

		for ent in flips {
			let ref mut keystate = data.keystate.get_mut(ent).unwrap();

			keystate.stealthed = !keystate.stealthed;

			data.channel.single_write(PlayerStealth {
				player: ent,
				stealthed: keystate.stealthed,
			});
		}
	}
}

impl SystemInfo for SetStealth {
	type Dependencies = (KeyHandler);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self {}
	}
}

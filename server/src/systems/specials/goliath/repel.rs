
use specs::*;
use types::*;
use types::systemdata::*;

use SystemInfo;

use component::channel::*;
use component::flag::IsPlayer;
use component::event::PlayerRepel;

use systems::EnergyRegenSystem;
use systems::handlers::packet::KeyHandler;
use systems::specials::config::*;

pub struct GoliathRepel;

#[derive(SystemData)]
pub struct GoliathRepelData<'a> {
	channel: Write<'a, OnPlayerRepel>,
	entities: Entities<'a>,

	keystate: ReadStorage<'a, KeyState>,
	energy: WriteStorage<'a, Energy>,
	plane: ReadStorage<'a, Plane>,
	is_player: ReadStorage<'a, IsPlayer>,
	is_alive: IsAlive<'a>,
}

impl<'a> System<'a> for GoliathRepel {
	type SystemData = GoliathRepelData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let mut channel = data.channel;

		(
			&*data.entities,
			&data.keystate,
			&mut data.energy,
			&data.plane,
			&data.is_player,
			data.is_alive.mask()
		).join()
			.filter(|(_, _, _, plane, ..)| **plane == Plane::Goliath)
			.filter(|(_, _, energy, ..)| **energy > *GOLIATH_SPECIAL_ENERGY)
			.filter(|(_, keystate, ..)| keystate.special)
			.for_each(|(ent, ..)| {
				channel.single_write(PlayerRepel{
					player: ent
				})
			});
	}
}

impl SystemInfo for GoliathRepel {
	type Dependencies = (
		EnergyRegenSystem,
		KeyHandler
	);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self{}
	}
}

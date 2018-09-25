use specs::*;
use types::systemdata::*;
use types::*;

use SystemInfo;

use component::channel::*;
use component::event::PlayerRepel;
use component::flag::IsPlayer;
use component::time::{LastRepelTime, ThisFrame};

use systems::handlers::packet::KeyHandler;
use systems::specials::config::*;
use systems::EnergyRegenSystem;

pub struct GoliathRepel;

#[derive(SystemData)]
pub struct GoliathRepelData<'a> {
	channel: Write<'a, OnPlayerRepel>,
	entities: Entities<'a>,
	this_frame: Read<'a, ThisFrame>,

	keystate: ReadStorage<'a, KeyState>,
	energy: WriteStorage<'a, Energy>,
	plane: ReadStorage<'a, Plane>,
	last_repel: WriteStorage<'a, LastRepelTime>,
	is_player: ReadStorage<'a, IsPlayer>,
	is_alive: IsAlive<'a>,
}

impl<'a> System<'a> for GoliathRepel {
	type SystemData = GoliathRepelData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let mut channel = data.channel;
		let this_frame = data.this_frame;

		(
			&*data.entities,
			&data.keystate,
			&mut data.energy,
			&data.plane,
			&mut data.last_repel,
			&data.is_player,
			data.is_alive.mask(),
		)
			.join()
			.filter(|(_, _, _, plane, ..)| **plane == Plane::Goliath)
			.filter(|(_, _, energy, ..)| **energy > *GOLIATH_SPECIAL_ENERGY)
			.filter(|(_, keystate, ..)| keystate.special)
			.filter(|(_, _, _, _, last_repel, ..)| {
				this_frame.0 - last_repel.0 > *GOLIATH_SPECIAL_INTERVAL
			}).for_each(|(ent, _, energy, _, last_repel, ..)| {
				channel.single_write(PlayerRepel { player: ent });

				*energy -= *GOLIATH_SPECIAL_ENERGY;
				*last_repel = LastRepelTime(this_frame.0);
			});
	}
}

impl SystemInfo for GoliathRepel {
	type Dependencies = (EnergyRegenSystem, KeyHandler);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self {}
	}
}

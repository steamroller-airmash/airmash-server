use server::*;
use specs::*;

use component::*;

use airmash_server::systems::handlers::game::on_join::AllJoinHandlers;

use super::PickupFlagSystem;

pub struct FlagSpeedSystem {
	reader: Option<OnFlagReader>,
}

#[derive(SystemData)]
pub struct FlagSpeedSystemData<'a> {
	pub channel: Read<'a, OnFlag>,

	pub keystate: WriteStorage<'a, KeyState>,
}

impl FlagSpeedSystem {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for FlagSpeedSystem {
	type SystemData = FlagSpeedSystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnFlag>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			channel,
			mut keystate,
		} = data;

		for evt in channel.read(self.reader.as_mut().unwrap()) {
			let player = evt.player.unwrap();
			let keystate = keystate.get_mut(player);

			if keystate.is_none() {
				continue;
			}

			let keystate = keystate.unwrap();

			match evt.ty {
				FlagEventType::Capture | FlagEventType::Drop => {
					keystate.flagspeed = false;
				}
				FlagEventType::PickUp => {
					keystate.flagspeed = true;
				}
				_ => (),
			}
		}
	}
}

impl SystemInfo for FlagSpeedSystem {
	type Dependencies = (PickupFlagSystem, AllJoinHandlers);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

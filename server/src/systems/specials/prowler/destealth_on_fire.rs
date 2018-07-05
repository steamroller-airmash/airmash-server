
use specs::*;
use types::*;

use SystemInfo;
use component::channel::*;
use systems::missile::MissileFireHandler;

pub struct DestealthOnFire {
	reader: Option<OnMissileFireReader>,
}

#[derive(SystemData)]
pub struct DestealthOnFireData<'a> {
	channel: Read<'a, OnMissileFire>,

	keystate: WriteStorage<'a, KeyState>,
	plane: ReadStorage<'a, Plane>
}

impl<'a> System<'a> for DestealthOnFire {
	type SystemData = DestealthOnFireData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(
			res.fetch_mut::<OnMissileFire>().register_reader()
		);
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if *data.plane.get(evt.player).unwrap() != Plane::Prowler {
				continue;
			}

			match data.keystate.get_mut(evt.player) {
				Some(keystate) => keystate.stealthed = false,
				_ => ()
			}
		}
	}
}

impl SystemInfo for DestealthOnFire {
	type Dependencies = MissileFireHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self{ reader: None }
	}
}

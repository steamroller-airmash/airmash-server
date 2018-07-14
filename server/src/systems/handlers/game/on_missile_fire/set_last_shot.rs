use specs::*;

use systems::missile::MissileFireHandler;
use SystemInfo;

use component::channel::*;
use component::time::{LastShotTime, ThisFrame};

pub struct SetLastShot {
	reader: Option<OnMissileFireReader>,
}

#[derive(SystemData)]
pub struct SetLastShotData<'a> {
	pub channel: Read<'a, OnMissileFire>,
	pub this_frame: Read<'a, ThisFrame>,

	pub last_shot: WriteStorage<'a, LastShotTime>,
}

impl SetLastShot {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for SetLastShot {
	type SystemData = SetLastShotData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnMissileFire>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			data.last_shot
				.insert(evt.player, LastShotTime(data.this_frame.0))
				.unwrap();
		}
	}
}

impl SystemInfo for SetLastShot {
	type Dependencies = MissileFireHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

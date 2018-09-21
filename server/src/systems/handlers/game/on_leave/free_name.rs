use specs::*;

use dispatch::SystemInfo;

use component::channel::*;
use component::collection::PlayerNames;

use systems::missile::MissileHit;

pub struct FreeName {
	reader: Option<OnPlayerLeaveReader>,
}

#[derive(SystemData)]
pub struct FreeNameData<'a> {
	pub channel: Read<'a, OnPlayerLeave>,
	pub player_names: Write<'a, PlayerNames>,
}

impl FreeName {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for FreeName {
	type SystemData = FreeNameData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerLeave>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			data.player_names.0.remove_by_value(&evt.0);
		}
	}
}

impl SystemInfo for FreeName {
	type Dependencies = (MissileHit, super::KnownEventSources);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

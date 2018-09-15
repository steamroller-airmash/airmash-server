use specs::*;

use dispatch::SystemInfo;

use component::channel::*;
use component::flag::IsSpectating;

pub struct SetSpectateFlag {
	reader: Option<OnPlayerSpectateReader>,
}

#[derive(SystemData)]
pub struct SetSpectateFlagData<'a> {
	pub channel: Read<'a, OnPlayerSpectate>,

	pub is_spec: WriteStorage<'a, IsSpectating>,
}

impl<'a> System<'a> for SetSpectateFlag {
	type SystemData = SetSpectateFlagData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerSpectate>().register_reader())
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			data.is_spec.insert(evt.player, IsSpectating).unwrap();
		}
	}
}

impl SystemInfo for SetSpectateFlag {
	type Dependencies = super::KnownEventSources;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

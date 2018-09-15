use specs::*;

use dispatch::SystemInfo;

use component::channel::*;
use component::reference::PlayerRef;

pub struct SetSpectateTarget {
	reader: Option<OnPlayerSpectateReader>,
}

#[derive(SystemData)]
pub struct SetSpectateTargetData<'a> {
	pub channel: Read<'a, OnPlayerSpectate>,

	pub spec_tgt: WriteStorage<'a, PlayerRef>,
}

impl<'a> System<'a> for SetSpectateTarget {
	type SystemData = SetSpectateTargetData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerSpectate>().register_reader())
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let target = match evt.target {
				Some(ent) => ent,
				None => evt.player,
			};

			data.spec_tgt.insert(evt.player, PlayerRef(target)).unwrap();
		}
	}
}

impl SystemInfo for SetSpectateTarget {
	type Dependencies = super::KnownEventSources;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

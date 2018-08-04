use specs::*;

use server::component::channel::*;
use server::*;

use consts::*;

#[derive(Default)]
pub struct RespawnAll {
	reader: Option<OnTimerEventReader>,
}

#[derive(SystemData)]
pub struct RespawnAllData<'a> {
	channel: Read<'a, OnTimerEvent>,
}

impl<'a> System<'a> for RespawnAll {
	type SystemData = RespawnAllData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnTimerEvent>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if evt.ty != *RESPAWN_TIMER {
				continue;
			}

			// TODO
		}
	}
}

impl SystemInfo for RespawnAll {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

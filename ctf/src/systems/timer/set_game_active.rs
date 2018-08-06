use specs::*;

use server::component::channel::*;
use server::*;

use component::*;
use consts::*;

/// Resets game score to 0-0 when the
/// game starts.
#[derive(Default)]
pub struct SetGameActive {
	reader: Option<OnTimerEventReader>,
}

#[derive(SystemData)]
pub struct SetGameActiveData<'a> {
	channel: Read<'a, OnTimerEvent>,
	game_active: Write<'a, GameActive>,
}

impl<'a> System<'a> for SetGameActive {
	type SystemData = SetGameActiveData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnTimerEvent>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if evt.ty != *SET_GAME_ACTIVE {
				continue;
			}

			data.game_active.0 = true;
		}
	}
}

impl SystemInfo for SetGameActive {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

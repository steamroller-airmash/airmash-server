use specs::*;

use server::component::channel::*;
use server::*;

use component::*;
use consts::*;

/// Routes the [`GAME_START_TIMER`] into a separate
/// event ([`OnGameStart`]).
#[derive(Default)]
pub struct GameStart {
	reader: Option<OnTimerEventReader>,
}

#[derive(SystemData)]
pub struct GameStartData<'a> {
	channel: Read<'a, OnTimerEvent>,
	game_start_channel: Write<'a, OnGameStart>,
}

impl<'a> System<'a> for GameStart {
	type SystemData = GameStartData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnTimerEvent>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if evt.ty != *GAME_START_TIMER {
				continue;
			}

			data.game_start_channel.single_write(GameStartEvent);
		}
	}
}

impl SystemInfo for GameStart {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

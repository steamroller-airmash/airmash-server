use specs::*;

use server::*;

use component::*;
use systems::timer::GameStart;

/// Resets game score to 0-0 when the
/// game starts.
#[derive(Default)]
pub struct ResetScore {
	reader: Option<OnGameStartReader>,
}

#[derive(SystemData)]
pub struct ResetScoreData<'a> {
	channel: Read<'a, OnGameStart>,
	scores: Write<'a, GameScores>,

	flags: ReadExpect<'a, Flags>,
	flag_channel: Write<'a, OnFlag>,
}

impl<'a> System<'a> for ResetScore {
	type SystemData = ResetScoreData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnGameStart>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for _ in data.channel.read(self.reader.as_mut().unwrap()) {
			*data.scores = GameScores {
				blueteam: 0,
				redteam: 0,
			};

			// TODO: Establish what the official server does
			data.flag_channel.single_write(FlagEvent {
				ty: FlagEventType::Return,
				flag: data.flags.red,
				player: None,
			});

			data.flag_channel.single_write(FlagEvent {
				ty: FlagEventType::Return,
				flag: data.flags.blue,
				player: None,
			});
		}
	}
}

impl SystemInfo for ResetScore {
	type Dependencies = GameStart;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

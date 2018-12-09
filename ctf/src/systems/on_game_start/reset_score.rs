use specs::*;

use server::utils::*;
use server::*;

use component::*;
use systems::timer::GameStart;

/// Resets game score to 0-0 when the
/// game starts.
#[derive(Default)]
pub struct ResetScore;

#[derive(SystemData)]
pub struct ResetScoreData<'a> {
	scores: Write<'a, GameScores>,

	flags: ReadExpect<'a, Flags>,
	flag_channel: Write<'a, OnFlag>,
}

impl EventHandlerTypeProvider for ResetScore {
	type Event = GameStartEvent;
}

impl<'a> EventHandler<'a> for ResetScore {
	type SystemData = ResetScoreData<'a>;

	fn on_event(&mut self, _: &GameStartEvent, data: &mut Self::SystemData) {
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

impl SystemInfo for ResetScore {
	type Dependencies = GameStart;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

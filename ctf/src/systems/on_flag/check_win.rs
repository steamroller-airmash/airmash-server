use specs::*;

use component::*;
use config::{BLUE_TEAM, RED_TEAM};
use server::*;

use super::SendFlagMessage;

#[derive(Default)]
pub struct CheckWin {
	reader: Option<OnFlagReader>,
}

#[derive(SystemData)]
pub struct CheckWinData<'a> {
	flag_channel: Read<'a, OnFlag>,
	win_channel: Write<'a, OnGameWin>,
	scores: Read<'a, GameScores>,
}

impl<'a> System<'a> for CheckWin {
	type SystemData = CheckWinData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnFlag>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.flag_channel.read(self.reader.as_mut().unwrap()) {
			// Ignore all non-capture events, this is to prevent
			// win events from being fired spuriously
			match evt.ty {
				FlagEventType::Capture => (),
				_ => continue,
			}

			// Check to see if the game is over yet
			if data.scores.redteam < 3 && data.scores.blueteam < 3 {
				continue;
			}

			let winning_team = if data.scores.redteam >= 3 {
				RED_TEAM
			} else {
				BLUE_TEAM
			};

			data.win_channel.single_write(GameWinEvent { winning_team });
		}
	}
}

impl SystemInfo for CheckWin {
	type Dependencies = SendFlagMessage;

	fn new() -> Self {
		Self::default()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

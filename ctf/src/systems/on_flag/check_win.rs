use specs::*;

use crate::component::*;
use crate::config::{BLUE_TEAM, RED_TEAM};
use crate::server::utils::*;
use crate::server::*;

use super::SendFlagMessage;

#[derive(Default)]
pub struct CheckWin;

#[derive(SystemData)]
pub struct CheckWinData<'a> {
	win_channel: Write<'a, OnGameWin>,
	scores: Read<'a, GameScores>,
}

impl EventHandlerTypeProvider for CheckWin {
	type Event = FlagEvent;
}

impl<'a> EventHandler<'a> for CheckWin {
	type SystemData = CheckWinData<'a>;

	fn on_event(&mut self, evt: &FlagEvent, data: &mut Self::SystemData) {
		// Ignore all non-capture events, this is to prevent
		// win events from being fired spuriously
		match evt.ty {
			FlagEventType::Capture => (),
			_ => return,
		}

		// Check to see if the game is over yet
		if data.scores.redteam < 3 && data.scores.blueteam < 3 {
			return;
		}

		let winning_team = if data.scores.redteam >= 3 {
			RED_TEAM
		} else {
			BLUE_TEAM
		};

		data.win_channel.single_write(GameWinEvent { winning_team });
	}
}

system_info! {
	impl SystemInfo for CheckWin {
		type Dependencies = SendFlagMessage;
	}
}

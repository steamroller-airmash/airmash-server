use std::sync::mpsc::Receiver;

use shrev::*;
use specs::*;
use types::*;

pub struct TimerHandler {
	channel: Receiver<TimerEvent>,
}

impl TimerHandler {
	pub fn new(channel: Receiver<TimerEvent>) -> Self {
		Self { channel }
	}
}

#[derive(SystemData)]
pub struct TimerHandlerData<'a> {
	pub scoreboard: Write<'a, EventChannel<ScoreBoardTimerEvent>>,
	pub afk_timer: Write<'a, EventChannel<AFKTimerEvent>>,
}

impl<'a> System<'a> for TimerHandler {
	type SystemData = TimerHandlerData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		while let Ok(evt) = self.channel.try_recv() {
			match evt.ty {
				TimerEventType::ScoreBoard => {
					data.scoreboard
						.single_write(ScoreBoardTimerEvent(evt.instant));
				}
				TimerEventType::AFKTimeout => {
					data.afk_timer.single_write(AFKTimerEvent(evt.instant));
				}
			}
		}
	}
}

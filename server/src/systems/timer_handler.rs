use std::mem;
use std::any::Any;
use std::sync::mpsc::{Receiver, channel};

use component::event::*;
use shrev::*;
use specs::*;
use types::event::*;
use dispatch::SystemInfo;

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
	pub ping_timer: Write<'a, EventChannel<PingTimerEvent>>,
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
				TimerEventType::PingDispatch => {
					data.ping_timer.single_write(PingTimerEvent(evt.instant));
				}
			}
		}
	}
}

impl SystemInfo for TimerHandler {
	type Dependencies = ();

	fn name() -> &'static str {
		module_path!()
	}

	fn new(mut a: Box<Any>) -> Self {
		let r = a.downcast_mut::<Receiver<TimerEvent>>().unwrap();
		// Replace the channel within the box with a 
		// dummy one, which will be dropped immediately
		// anyway
		Self::new(mem::replace(r, channel().1))
	}
}

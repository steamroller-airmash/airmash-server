use std::sync::mpsc::Sender;
use std::time::Duration;

use timeloop::timeloop;
use types::event::*;

use tokio;

pub fn start_timer_events(channel: Sender<TimerEvent>) {
	// 5s timer for ScoreBoard
	tokio::spawn({
		let channel = channel.clone();
		timeloop(
			move |instant| {
				channel
					.send(TimerEvent {
						ty: TimerEventType::ScoreBoard,
						instant: instant,
					})
					.unwrap();
			},
			Duration::from_secs(5),
		)
	});

	// 250ms timer for ping, in the official 
	// server this goes at 50ms, but that isn't
	// necessary for now
	tokio::spawn({
		let channel = channel.clone();
		timeloop(
			move |instant| {
				channel.send(TimerEvent {
					ty: TimerEventType::PingDispatch,
					instant: instant
				})
				.unwrap();
			},
			Duration::from_millis(250)
		)
	});
}

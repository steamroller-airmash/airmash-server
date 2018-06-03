use std::sync::mpsc::Sender;
use std::time::Duration;

use timeloop::timeloop;
use types::*;

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

	// 50ms timer for Ping
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
			Duration::from_millis(50)
		)
	});
}

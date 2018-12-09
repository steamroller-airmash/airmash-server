use std::sync::mpsc::Sender;
use std::time::Duration;

use super::timeloop::timeloop;
use component::event::TimerEvent;
use consts::timer::*;

use tokio;

pub fn start_timer_events(channel: Sender<TimerEvent>) {
	// 5s timer for ScoreBoard
	tokio::spawn({
		let channel = channel.clone();
		timeloop(
			move |instant| {
				channel
					.send(TimerEvent {
						ty: *SCORE_BOARD,
						instant: instant,
						..Default::default()
					})
					.unwrap();
				true
			},
			Duration::from_secs(5),
		)
	});
}

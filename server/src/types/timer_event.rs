use std::time::Instant;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TimerEventType {
	ScoreBoard,
	AFKTimeout,
	PingDispatch,
}

#[derive(Copy, Clone, Debug)]
pub struct TimerEvent {
	pub instant: Instant,
	pub ty: TimerEventType,
}

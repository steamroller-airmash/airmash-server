use utils::timer::*;

lazy_static! {
	pub static ref INVALID: TimerEventType = register_event_type();
	pub static ref PING_DISPATCH: TimerEventType = register_event_type();
	pub static ref AFK_TIMER: TimerEventType = register_event_type();
	pub static ref SCORE_BOARD: TimerEventType = register_event_type();
	pub static ref RESPAWN_TIME: TimerEventType = register_event_type();
	pub static ref UNTHROTTLE_TIME: TimerEventType = register_event_type();
	pub static ref LOGIN_PASSED: TimerEventType = register_event_type();
	pub static ref LOGIN_FAILED: TimerEventType = register_event_type();
}

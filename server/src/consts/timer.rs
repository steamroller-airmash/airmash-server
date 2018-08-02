use utils::timer::*;

lazy_static! {
	pub static ref INVALID: TimerEventType = TimerEventType::register();
	pub static ref PING_DISPATCH: TimerEventType = TimerEventType::register();
	pub static ref AFK_TIMER: TimerEventType = TimerEventType::register();
	pub static ref SCORE_BOARD: TimerEventType = TimerEventType::register();
	pub static ref RESPAWN_TIME: TimerEventType = TimerEventType::register();
	pub static ref UNTHROTTLE_TIME: TimerEventType = TimerEventType::register();
	pub static ref LOGIN_PASSED: TimerEventType = TimerEventType::register();
	pub static ref LOGIN_FAILED: TimerEventType = TimerEventType::register();
	pub static ref DELAYED_MESSAGE: TimerEventType = TimerEventType::register();
}

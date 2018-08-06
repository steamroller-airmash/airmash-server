use server::component::event::TimerEventType;

lazy_static! {
	pub static ref RESTORE_CONFIG: TimerEventType = TimerEventType::register();
	pub static ref GAME_START_TIMER: TimerEventType = TimerEventType::register();
	pub static ref RETEAM_TIMER: TimerEventType = TimerEventType::register();
	pub static ref SET_GAME_ACTIVE: TimerEventType = TimerEventType::register();
}

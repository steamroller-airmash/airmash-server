use server::component::event::TimerEventType;

lazy_static! {
	pub static ref RESTORE_CONFIG: TimerEventType = TimerEventType::register();
}


use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

static TIMER_EVENT_TYPE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TimerEventType(usize);

pub fn register_event_type() -> TimerEventType {
	TimerEventType(
		TIMER_EVENT_TYPE_COUNTER.fetch_add(1, Ordering::Relaxed)
	)
}

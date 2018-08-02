use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static TIMER_EVENT_TYPE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TimerEventType(usize);

fn register_event_type() -> TimerEventType {
	TimerEventType(TIMER_EVENT_TYPE_COUNTER.fetch_add(1, Ordering::Relaxed))
}

impl TimerEventType {
	pub fn register() -> Self {
		register_event_type()
	}
}

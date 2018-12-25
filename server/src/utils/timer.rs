//! `TimerEventType`

use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static TIMER_EVENT_TYPE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;

/// An identifier for different types of timer events.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TimerEventType(usize);

fn register_event_type() -> TimerEventType {
	TimerEventType(TIMER_EVENT_TYPE_COUNTER.fetch_add(1, Ordering::Relaxed))
}

impl TimerEventType {
	/// Register a new unique timer event type.
	///
	/// Ideally this should be done with a `lazy_static!`
	/// or similar so that all systems that need the timer
	/// id can get at the same one.
	pub fn register() -> Self {
		register_event_type()
	}
}

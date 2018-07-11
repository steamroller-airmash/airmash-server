use bounded_queue::BoundedQueue;

use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct RateLimiter {
	window: Duration,
	events: BoundedQueue<Instant>,
}

impl RateLimiter {
	pub fn new(max_events: usize, window: Duration) -> Self {
		Self {
			window,
			events: BoundedQueue::new(max_events + 1),
		}
	}

	pub fn update(&mut self, now: Instant) {
		let prev = now - self.window;

		while let Some(&inst) = self.events.peek() {
			if inst < prev {
				self.events.pop();
			} else {
				break;
			}
		}
	}

	pub fn add_event(&mut self, now: Instant) {
		self.update(now);
		self.events.push(now);
	}

	pub fn limit_reached(&self) -> bool {
		self.events.is_full()
	}
}

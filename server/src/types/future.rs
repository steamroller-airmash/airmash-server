use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use component::event::TimerEvent;

struct Task {
	pub time: Instant,
	pub func: Box<Fn(Instant) -> Option<TimerEvent> + Send>,
}

impl PartialEq for Task {
	fn eq(&self, o: &Self) -> bool {
		self.time.eq(&o.time)
	}
}

impl Eq for Task {}

impl PartialOrd for Task {
	fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
		Reverse(self.time).partial_cmp(&Reverse(o.time))
	}
}

impl Ord for Task {
	fn cmp(&self, o: &Self) -> Ordering {
		Reverse(self.time).cmp(&Reverse(o.time))
	}
}

pub struct FutureDispatcher {
	channel: Mutex<Sender<TimerEvent>>,
	tasks: Mutex<BinaryHeap<Task>>,
}

/// Allow running delayed tasks
impl FutureDispatcher {
	pub fn new(channel: Sender<TimerEvent>) -> Self {
		Self {
			channel: Mutex::new(channel),
			tasks: Default::default(),
		}
	}

	/// Retrieves a clone of the timer event channel
	pub fn get_channel(&self) -> Sender<TimerEvent> {
		self.channel.lock().unwrap().clone()
	}

	/// Runs the function after the
	/// timeout has completed
	pub fn run_delayed<F: 'static>(&self, dur: Duration, fun: F)
	where
		F: Send + Fn(Instant) -> Option<TimerEvent>,
	{
		let instant = Instant::now() + dur;

		self.tasks.lock().unwrap().push(Task {
			time: instant,
			func: Box::new(fun),
		});
	}

	pub fn exec_tasks(&mut self, now: Instant) {
		let tasks = self.tasks.get_mut().unwrap();

		while !tasks.is_empty() && tasks.peek().unwrap().time < now {
			let task = tasks.pop().unwrap();

			if let Some(evt) = (task.func)(now) {
				self.channel.get_mut().unwrap().send(evt).unwrap();
			}
		}
	}
}

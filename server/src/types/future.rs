use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::mem;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use component::event::TimerEvent;

struct Task {
	pub time: Instant,
	pub func: Box<FnMut(Instant) -> Option<TimerEvent> + Send>,
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
	pub fn run_delayed<F: 'static, I>(&self, dur: Duration, fun: F)
	where
		F: Send + FnOnce(Instant) -> I,
		I: Into<Option<TimerEvent>>,
	{
		let instant = Instant::now() + dur;
		let mut opt = Some(fun);

		self.tasks.lock().unwrap().push(Task {
			time: instant,
			func: Box::new(move |inst| {
				// This is a little bit hack, but since
				// it is impossible to call a Box<FnOnce(...) -> ...>
				// in stable rust without enabling yet another
				// nightly extension this will have to do instead.
				let fun =
					mem::replace(&mut opt, None).expect("Delayed task was executed more than once");

				fun(inst).into()
			}),
		});
	}

	pub fn exec_tasks(&mut self, now: Instant) {
		let tasks = self.tasks.get_mut().unwrap();

		while !tasks.is_empty() && tasks.peek().unwrap().time < now {
			let mut task = tasks.pop().unwrap();

			if let Some(evt) = (task.func)(now) {
				self.channel.get_mut().unwrap().send(evt).unwrap();
			}
		}
	}
}

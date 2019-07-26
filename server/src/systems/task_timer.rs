use specs::{System, Read, Resources, SystemData};
use shrev::{EventChannel, ReaderId};

use crate::component::time::ThisFrame;
use crate::utils::MaybeInit;

use std::task::Waker;
use std::collections::BinaryHeap;
use std::time::Instant;
use std::cmp::{Ordering, Reverse};

struct Task {
	time: Instant,
	waker: Waker
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

/// Not declared in components since this this is
/// an internal implementation detail.
pub struct WakerEvent(pub Instant, pub Waker);
pub type WakerChannel = EventChannel<WakerEvent>;

#[derive(SystemData)]
pub struct TaskTimerData<'a> {
	frame: Read<'a, ThisFrame>,
	events: Read<'a, WakerChannel>
}

#[derive(Default)]
pub struct TaskTimerSystem {
	queued: BinaryHeap<Task>,
	reader: MaybeInit<ReaderId<WakerEvent>>
}

impl<'a> System<'a> for TaskTimerSystem {
	type SystemData = TaskTimerData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);
		res.insert(WakerChannel::with_capacity(500));

		self.reader = MaybeInit::new(
			res.fetch_mut::<WakerChannel>().register_reader()
		);
	}

	fn run(&mut self, data: TaskTimerData<'a>) {
		let this_frame = data.frame.0;

		for WakerEvent(time, waker) in data.events.read(&mut self.reader) {
			self.queued.push(Task { 
				time: *time, 
				waker: waker.clone()
			});
		}

		while self.queued.peek().map(|t| t.time < this_frame).unwrap_or(false) {
			let task = self.queued.pop().unwrap();
			task.waker.wake();
		}
	}
}

system_info! {
	impl SystemInfo for TaskTimerSystem {
		type Dependencies = ();
	}
}

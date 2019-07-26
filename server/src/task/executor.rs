
use std::task::{RawWaker, RawWakerVTable};
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::pin::Pin;
use hashbrown::HashMap;

type TaskId = u64;

#[derive(Clone)]
struct WakerData {
	exec: Arc<Mutex<ExecutorQueue>>,
	id: TaskId
}

mod waker_vtable {
	use super::*;

	pub(super) const VTABLE: RawWakerVTable = RawWakerVTable::new(
		clone, wake, wake, drop
	);

	unsafe fn clone(arcptr: *const ()) -> RawWaker {
		let data = &*(arcptr as *const WakerData);
		let newptr = Box::into_raw(Box::new(data.clone()));

		RawWaker::new(newptr as *const (), &VTABLE)
	}
	unsafe fn wake(arcptr: *const ()) {
		let data = &*(arcptr as *const WakerData);

		let mut lock = data.exec.lock().unwrap();

		lock.wake_task(data.id);
	}
	unsafe fn drop(arcptr: *const ()) {
		let _ = Box::from_raw(arcptr as *mut WakerData);
	}
}

type BoxedFuture = Box<dyn Future<Output = ()> + Send + 'static>;
type PinnedFuture = Pin<BoxedFuture>;

// Safety note: 
// The BoxedFutures within the hash map should never
// have their inner values moved as they are pinned.
#[derive(Default)]
struct ExecutorData {
	tasks: HashMap<TaskId, PinnedFuture>
}

#[derive(Default)]
struct ExecutorQueue {
	nextid: TaskId,
	to_wake: Vec<TaskId>,
	tasks: Vec<(TaskId, BoxedFuture)>,
}

impl ExecutorQueue {
	fn wake_task(&mut self, id: TaskId) {
		self.to_wake.push(id)
	}

	fn alloc_task(&mut self) -> TaskId {
		let id = self.nextid;
		self.nextid += 1;
		id
	}

	fn create_task(&mut self, task: BoxedFuture) -> TaskId {
		let id = self.alloc_task();

		self.tasks.push((id, task));
		self.to_wake.push(id);

		id
	}
}

impl ExecutorData {
	fn start_task(&mut self, id: TaskId, task: BoxedFuture) {
		self.tasks.insert(id, Box::into_pin(task));
	}

	fn delete_task(&mut self, task: TaskId) {
		self.tasks.remove(&task);
	}

	fn poll_all(&mut self, tasks: &[TaskId], queue: &Arc<Mutex<ExecutorQueue>>) {
		use std::task::{Waker, Context, Poll};

		for taskid in tasks {
			let data = Box::into_raw(Box::new(WakerData{ 
				exec: Arc::clone(queue),
				id: *taskid
			}));
			let raw = RawWaker::new(data as *const (), &waker_vtable::VTABLE);

			let res = unsafe {
				let waker = Waker::from_raw(raw);
				let mut context = Context::from_waker(&waker);

				let task = match self.tasks.get_mut(taskid) {
					Some(task) => task,
					// Task got deleted in a previous iteration most likely
					None => continue,
				};

				Future::poll(task.as_mut(), &mut context)
			};

			match res {
				Poll::Ready(()) => self.delete_task(*taskid),
				_ => ()
			}
		}
	}
}

#[derive(Clone)]
pub struct ExecutorHandle {
	inner: Arc<Mutex<ExecutorData>>,
	queue: Arc<Mutex<ExecutorQueue>>,
}

impl ExecutorHandle {
	/// Create a new executor and return a handle to it
	pub fn new() -> Self {
		Self {
			inner: Default::default(),
			queue: Default::default()
		}
	}

	/// Spawn a new future given a boxed future
	pub fn spawn_fut_dyn(&mut self, fut: Box<dyn Future<Output = ()> + Send + 'static>) {
		self.queue.lock().unwrap().create_task(fut);
	}
	/// Spawn a new future given a future object
	pub fn spawn_fut<F>(&mut self, fut: F) 
	where
		F: Future<Output = ()> + Send + 'static
	{
		self.spawn_fut_dyn(Box::new(fut));
	}

	/// Poll all futures which have requested to be so polled
	pub fn cycle(&mut self) {
		// Implementation note: 
		//  The scopes here are set up so that we are only
		//  holding at most one lock at any given time. If
		//  we hold both then we would deadlock if a future
		//  calls Waker::wake() while it's being polled. 

		let (mut to_awaken, new_tasks) = {
			let mut queue = self.queue.lock().unwrap();
			(
				std::mem::replace(&mut queue.to_wake, vec![]),
				std::mem::replace(&mut queue.tasks, vec![])
			)
		};

		{
			let mut inner = self.inner.lock().unwrap();

			for (id, task) in new_tasks {
				inner.start_task(id, task);
			}

			inner.poll_all(&to_awaken, &self.queue);
		}

		to_awaken.clear();
		// Put back the original vector. If nothing signalled
		// that it should be polled then this will save an
		// allocation
		let mut queue = self.queue.lock().unwrap();
		std::mem::swap(&mut to_awaken, &mut queue.to_wake);
		// However, if something did request to be polled then
		// we still need to put it in the \queue
		queue.to_wake.extend_from_slice(&to_awaken);
	}
}


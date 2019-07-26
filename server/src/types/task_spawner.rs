
use crate::task::{ExecutorHandle, TaskData};

use std::future::Future;

pub struct TaskSpawner {
	data: TaskData,
	handle: ExecutorHandle
}

impl TaskSpawner {
	pub fn new(data: TaskData, handle: ExecutorHandle) -> Self {
		Self {
			data,
			handle
		}
	}

	fn task_data(&self) -> TaskData {
		self.data.clone()
	}

	pub fn launch<T, F>(&mut self, task: T) 
	where
		T: FnOnce(TaskData) -> F,
		F: Future<Output = ()> + Send + 'static
	{
		self.handle.spawn_fut(task(self.task_data()));
	}
}



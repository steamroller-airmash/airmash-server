use crate::task::{ExecutorHandle, TaskData};

use std::future::Future;

pub struct TaskSpawner {
	data: TaskData,
	handle: ExecutorHandle,
}

impl TaskSpawner {
	pub fn new(data: TaskData, handle: ExecutorHandle) -> Self {
		Self { data, handle }
	}

	pub fn task_data(&self) -> TaskData {
		self.data.clone()
	}

	pub fn launch<F>(&mut self, fut: F)
	where
		F: Future<Output = ()> + Send + 'static,
	{
		self.handle.spawn_fut(fut);
	}
}

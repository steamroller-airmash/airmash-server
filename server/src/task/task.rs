use shred::{Fetch, FetchMut, Resource};
use specs::error::WrongGeneration;
use specs::{Component, Entity, ReadStorage, World, WriteStorage};

use parking_lot::RwLock;
use std::sync::Arc;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

/// This is a task's reference to the rest of the game world.
#[derive(Clone)]
pub struct TaskData {
	world: Arc<RwLock<World>>,
}

impl TaskData {
	pub fn new(world: Arc<RwLock<World>>) -> Self {
		Self { world }
	}

	/// Fetch a component by value.
	///
	/// # Panics
	/// Panics if `T` has not been registered as a component.
	pub fn fetch<T: Component + Clone>(&self, id: Entity) -> Option<T> {
		self.read_storage(move |storage| storage.get(id).cloned())
	}

	/// Analogous to `World::read_storage()` with the limitation
	/// that the storage can only be accessed within the callback.
	pub fn read_storage<T, F, R>(&self, cb: F) -> R
	where
		T: Component,
		F: FnOnce(ReadStorage<T>) -> R,
	{
		cb(self.world.read().read_storage::<T>())
	}

	/// Analogous to `World::write_storage()` with the limitation
	/// that the storage can only be accessed within the callback.
	pub fn write_storage<T, F, R>(&self, cb: F) -> R
	where
		T: Component,
		F: FnOnce(WriteStorage<T>) -> R,
	{
		cb(self.world.read().write_storage::<T>())
	}

	/// Analogous to `World::read_resource()` with the limitation
	/// that the resource can only be accessed within the callback.
	pub fn read_resource<T, F, R>(&self, cb: F) -> R
	where
		T: Resource,
		F: FnOnce(Fetch<T>) -> R,
	{
		cb(self.world.read().read_resource())
	}

	/// Analogous to `World::read_resource()` with the limitation
	/// that the resource can only be accessed within the callback.
	pub fn write_resource<T, F, R>(&self, cb: F) -> R
	where
		T: Resource,
		F: FnOnce(FetchMut<T>) -> R,
	{
		cb(self.world.read().write_resource())
	}

	/// Analogous to `World::create_entity()` with the limitation
	/// that the `EntityBuilder` can only be accessed within the callback.
	pub fn create_entity<F, R>(&mut self, cb: F) -> R
	where
		F: FnOnce(specs::EntityBuilder) -> R,
	{
		cb(self.world.write().create_entity())
	}

	/// Delete an entity and its components
	pub fn delete_entity(&mut self, entity: Entity) -> Result<(), WrongGeneration> {
		self.world.write().delete_entity(entity)
	}

	/// Delete all entities and their components
	pub fn delete_all(&mut self) {
		self.world.write().delete_all()
	}

	/// Returns a future that will resolve once the
	/// specified duration has passed.
	pub fn sleep_for<'a>(&'a self, duration: Duration) -> impl Future<Output = ()> + 'a {
		self.sleep_until(Instant::now() + duration)
	}

	/// Returns a future that will resolve once
	/// `Instant::now() > instant`
	pub fn sleep_until<'a>(&'a self, instant: Instant) -> impl Future<Output = ()> + 'a {
		TimedFuture::new(self, instant)
	}

	/// Suspend the task until the next frame.
	pub fn yield_frame(&self) -> impl Future<Output = ()> {
		InstantFuture::default()
	}
}

/// A future that depends on the `TaskTimerSystem`
/// to wake it at the right time.
struct TimedFuture<'a> {
	data: &'a TaskData,
	end: Instant,
}

impl<'a> TimedFuture<'a> {
	fn new(data: &'a TaskData, end: Instant) -> Self {
		Self { data, end }
	}
}

impl Future for TimedFuture<'_> {
	type Output = ();

	fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<()> {
		use crate::systems::task_timer::{WakerChannel, WakerEvent};

		if Instant::now() > self.end {
			return Poll::Ready(());
		}

		self.data
			.write_resource::<WakerChannel, _, _>(|mut channel| {
				channel.single_write(WakerEvent(self.end, ctx.waker().clone()));
			});

		Poll::Pending
	}
}

/// A future that waits once then returns `Poll::Ready`
#[derive(Default)]
struct InstantFuture {
	ready: bool,
}

impl Future for InstantFuture {
	type Output = ();

	fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<()> {
		if self.ready {
			Poll::Ready(())
		} else {
			ctx.waker().wake_by_ref();
			self.ready = true;
			Poll::Pending
		}
	}
}

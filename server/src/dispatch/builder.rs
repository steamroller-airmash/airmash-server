use specs::*;

use std::any::Any;

use dispatch::sysbuilder::*;
use dispatch::sysinfo::*;
use dispatch::syswrapper::*;

use utils::event_handler::{EventHandler, EventHandlerTypeProvider};

pub struct Builder<'a, 'b> {
	builder: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> Builder<'a, 'b> {
	pub fn new() -> Self {
		Self {
			builder: DispatcherBuilder::new(),
		}
	}

	/// Add a new system to be scheduled.
	///
	/// The system's dependencies will be automatically
	/// determined from its implementation of the
	/// [`SystemInfo`] trait.
	pub fn with<T>(self) -> Self
	where
		T: for<'c> System<'c> + Send + SystemInfo + 'a,
		T::Dependencies: SystemDeps,
	{
		self.with_args::<T, ()>(())
	}

	/// Add a new system to be scheduled with a specified
	/// argument.
	///
	/// The system's dependencies will be automatically
	/// determined from its implementation of the
	/// [`SystemInfo`] trait.
	pub fn with_args<T, U: Any>(self, args: U) -> Self
	where
		T: for<'c> System<'c> + Send + SystemInfo + 'a,
		T::Dependencies: SystemDeps,
	{
		debug!("{} {:?}", T::name(), T::Dependencies::dependencies());
		Self {
			builder: self.builder.with(
				SystemWrapper(T::new_args(Box::new(args))),
				T::name(),
				&T::Dependencies::dependencies(),
			),
		}
	}

	pub fn with_handler<T>(self) -> Self
	where
		T: for<'c> EventHandler<'c> + EventHandlerTypeProvider + Send + Sync + SystemInfo + 'a,
	{
		self.with_handler_args::<T, ()>(())
	}

	pub fn with_handler_args<T, U: Any>(self, args: U) -> Self
	where
		T: for<'c> EventHandler<'c> + EventHandlerTypeProvider + Send + Sync + SystemInfo + 'a,
		T::Event: Send + Sync,
	{
		use utils::event_handler::EventHandlerWrapper;
		self.with_args::<EventHandlerWrapper<T>, U>(args)
	}

	/// Call the passed in function with self and
	/// return whatever the function returns.
	///
	/// This is meant as an ease-of-use wrapper
	/// for `register` style functions.
	pub fn with_registrar<F>(self, fun: F) -> Self
	where
		F: FnOnce(Self) -> Self,
	{
		fun(self)
	}

	/// Add a thread-local system.
	///
	/// Note that thread-local systems are
	/// executed in the order that they are added.
	pub fn with_thread_local<T: 'static>(self) -> Self
	where
		T: for<'c> System<'c> + SystemInfo + 'b,
	{
		self.with_thread_local_args::<T, _>(())
	}

	pub fn with_thread_local_args<T: 'static, U: Any>(self, args: U) -> Self
	where
		T: for<'c> System<'c> + SystemInfo + 'b,
	{
		Self {
			builder: SystemBuilder::<T>::new(args).build_thread_local(self.builder),
		}
	}

	pub fn inner(self) -> DispatcherBuilder<'a, 'b> {
		self.builder
	}

	pub fn build(self) -> Dispatcher<'a, 'b> {
		self.builder.build()
	}
}

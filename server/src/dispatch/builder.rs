use specs::*;

use std::any::Any;

use dispatch::sysbuilder::*;
use dispatch::sysinfo::*;
use dispatch::syswrapper::*;

pub struct Builder<'a, 'b> {
	builder: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> Builder<'a, 'b> {
	pub fn new() -> Self {
		Self {
			builder: DispatcherBuilder::new(),
		}
	}

	pub fn with<T>(self) -> Self
	where
		T: for<'c> System<'c> + Send + SystemInfo + 'a,
		T::Dependencies: SystemDeps,
	{
		self.with_args::<T, ()>(())
	}

	pub fn with_args<T, U: Any>(self, args: U) -> Self
	where
		T: for<'c> System<'c> + Send + SystemInfo + 'a,
		T::Dependencies: SystemDeps,
	{
		Self {
			builder: self.builder
				.with(
					SystemWrapper(T::new(Box::new(args))),
					T::name(),
					&T::Dependencies::dependencies()
				)
				//	SystemBuilder::<SystemWrapper<T>>::new(args)
				//.build(self.builder),
		}
	}

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

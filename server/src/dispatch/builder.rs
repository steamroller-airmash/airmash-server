
use specs::*;

use std::any::Any;

use dispatch::sysinfo::*;
use dispatch::sysbuilder::*;

pub struct Builder<'a, 'b> {
	builder: DispatcherBuilder<'a, 'b>
}

impl<'a, 'b> Builder<'a, 'b> {
	pub fn new() -> Self {
		Self {
			builder: DispatcherBuilder::new()
		}
	}

	pub fn with<T: 'static>(self) -> Self 
	where 
		T: SystemInfo + SystemDeps + Send + for<'c> System<'c>,
		for<'d> <T as System<'d>>::SystemData: SystemData<'d>
	{
		Self {
			builder: SystemBuilder::<T>::new(()).build(self.builder)
		}
	}
	
	pub fn with_args<T: 'static, U: Any>(self, args: U) -> Self 
	where 
		T: SystemInfo + SystemDeps + Send + for<'c> System<'c>,
		for<'d> <T as System<'d>>::SystemData: SystemData<'d>
	{
		Self {
			builder: SystemBuilder::<T>::new(args).build(self.builder)
		}
	}

	pub fn with_thread_local<T: 'static>(self) -> Self 
	where 
		T: SystemInfo + SystemDeps + Send + for<'c> System<'c>,
		for<'d> <T as System<'d>>::SystemData: SystemData<'d>
	{
		Self {
			builder: SystemBuilder::<T>::new(()).build_thread_local(self.builder)
		}
	}
	
	pub fn with_thread_local_args<T: 'static, U: Any>(self, args: U) -> Self 
	where 
		T: SystemInfo + SystemDeps + Send + for<'c> System<'c>,
		for<'d> <T as System<'d>>::SystemData: SystemData<'d>
	{
		Self {
			builder: SystemBuilder::<T>::new(args).build_thread_local(self.builder)
		}
	}

	pub fn inner(self) -> DispatcherBuilder<'a, 'b> {
		self.builder
	}

	pub fn build(self) -> Dispatcher<'a, 'b> {
		self.builder.build()
	}
}
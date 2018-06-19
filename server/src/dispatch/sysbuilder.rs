use std::any::Any;
use std::marker::PhantomData;

use specs::*;

use dispatch::sysinfo::*;
use dispatch::syswrapper::*;

pub trait AbstractBuilder {
	fn build<'a, 'b>(self, disp: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b>;

	fn build_thread_local<'a, 'b>(
		self,
		disp: DispatcherBuilder<'a, 'b>,
	) -> DispatcherBuilder<'a, 'b>;
}

pub struct SystemBuilder<T> {
	args: Box<Any>,
	marker: PhantomData<T>,
}

impl<T: SystemInfo> SystemBuilder<T> {
	pub fn new<U: Any>(args: U) -> Self {
		Self {
			args: Box::new(args),
			marker: PhantomData {},
		}
	}
}

impl<T: 'static> AbstractBuilder for SystemBuilder<T>
where
	T: SystemInfo + SystemDeps + Send + for<'c> System<'c>,
	for<'d> <T as System<'d>>::SystemData: SystemData<'d>,
{
	fn build<'a, 'b>(self, disp: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
		disp.with(
			SystemWrapper(T::new(self.args)),
			T::name(),
			&T::dependencies(),
		)
	}

	fn build_thread_local<'a, 'b>(
		self,
		disp: DispatcherBuilder<'a, 'b>,
	) -> DispatcherBuilder<'a, 'b> {
		disp.with_thread_local(SystemWrapper(T::new(self.args)))
	}
}

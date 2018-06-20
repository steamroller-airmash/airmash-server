use std::any::Any;
use std::marker::PhantomData;

use specs::*;

use dispatch::sysinfo::*;

pub trait AbstractBuilder<'a> {
	fn build<'b>(self, disp: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b>;
}

pub trait AbstractThreadLocalBuilder<'b> {
	fn build_thread_local<'a>(self, disp: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b>;
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

impl<'a, T> AbstractBuilder<'a> for SystemBuilder<T>
where
	T: for<'c> System<'c> + Send + 'a,
	T: SystemInfo,
	T::Dependencies: SystemDeps,
{
	fn build<'b>(self, disp: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
		disp.with(
			T::new(self.args),
			T::name(),
			&T::Dependencies::dependencies(),
		)
	}
}

impl<'b, T> AbstractThreadLocalBuilder<'b> for SystemBuilder<T>
where
	T: for<'c> System<'c> + 'b,
	T: SystemInfo,
{
	fn build_thread_local<'a>(self, disp: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
		disp.with_thread_local(T::new(self.args))
	}
}

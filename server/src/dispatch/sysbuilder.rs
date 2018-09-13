use std::any::Any;
use std::marker::PhantomData;
use std::mem;

use specs::*;

use dispatch::sysinfo::*;
use dispatch::syswrapper::*;

pub trait AbstractBuilder {
	fn build<'a, 'b>(&mut self, disp: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b>;
	fn name(&self) -> &'static str;
	fn deps(&self) -> Vec<&'static str>;
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

impl<T> AbstractBuilder for SystemBuilder<T>
where
	T: for<'c> System<'c> + SystemInfo + Send + 'static,
{
	fn build<'a, 'b>(&mut self, disp: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
		let args = mem::replace(&mut self.args, Box::new(()));
		disp.with(
			SystemWrapper(T::new_args(args)),
			T::name(),
			&T::Dependencies::dependencies(),
		)
	}

	fn name(&self) -> &'static str {
		T::name()
	}

	fn deps(&self) -> Vec<&'static str> {
		T::Dependencies::dependencies()
	}
}

impl<'b, T> AbstractThreadLocalBuilder<'b> for SystemBuilder<T>
where
	T: for<'c> System<'c> + 'b,
	T: SystemInfo,
{
	fn build_thread_local<'a>(self, disp: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
		disp.with_thread_local(T::new_args(self.args))
	}
}

use specs::*;

use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::mem;

use log::Level::Debug;

use dispatch::sysbuilder::*;
use dispatch::sysinfo::*;

use utils::event_handler::{EventHandler, EventHandlerTypeProvider};

pub struct Builder<'a, 'b> {
	builder: DispatcherBuilder<'a, 'b>,
	sysmap: HashMap<&'static str, Box<AbstractBuilder>>,
}

impl<'a, 'b> Builder<'a, 'b> {
	pub fn new() -> Self {
		Self {
			builder: DispatcherBuilder::new(),
			sysmap: HashMap::default(),
		}
	}

	/// Add a new system to be scheduled.
	///
	/// The system's dependencies will be automatically
	/// determined from its implementation of the
	/// [`SystemInfo`] trait.
	pub fn with<T>(self) -> Self
	where
		T: for<'c> System<'c> + Send + SystemInfo + 'static,
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
	pub fn with_args<T, U: Any>(mut self, args: U) -> Self
	where
		T: for<'c> System<'c> + Send + SystemInfo + 'static,
		T::Dependencies: SystemDeps,
	{
		trace!(
			target: "airmash:builder", 
			"{} {:?}", 
			T::name(),
			T::Dependencies::dependencies()
		);

		self.sysmap
			.insert(T::name(), Box::new(SystemBuilder::<T>::new(args)));

		self
	}

	pub fn with_handler<T>(self) -> Self
	where
		T: for<'c> EventHandler<'c> + EventHandlerTypeProvider + Send + Sync + SystemInfo + 'static,
	{
		self.with_handler_args::<T, ()>(())
	}

	pub fn with_handler_args<T, U: Any>(self, args: U) -> Self
	where
		T: for<'c> EventHandler<'c> + EventHandlerTypeProvider + Send + Sync + SystemInfo + 'static,
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
			..self
		}
	}

	fn build_with_all(&mut self) {
		let systems = self.system_toposort();
		let builder = mem::replace(&mut self.builder, DispatcherBuilder::new());

		if log_enabled!(Debug) {
			for sys in &systems {
				debug!(
					target: "airmash:builder",
					"Added system to builder: {name}",
					name = sys.name(),
				);
			}
		}

		self.builder = systems
			.into_iter()
			.rev()
			.fold(builder, |builder, mut sys| sys.build(builder));
	}

	pub fn inner(mut self) -> DispatcherBuilder<'a, 'b> {
		self.build_with_all();
		self.builder
	}

	pub fn build(mut self) -> Dispatcher<'a, 'b> {
		self.build_with_all();
		self.builder.build()
	}
}

// This impl is related to finding a toposort of
// the systems so that they can be registered in
// the correct order.
impl<'a, 'b> Builder<'a, 'b> {
	/// Get the names of all systems
	fn get_system_names(&self) -> HashSet<&'static str> {
		self.sysmap.keys().map(|&x| x).collect()
	}

	/// Find all systems that have no other systems
	/// depending on them
	fn find_roots(&self) -> HashSet<&'static str> {
		let mut names = self.get_system_names();

		for builder in self.sysmap.values() {
			for dep in builder.deps() {
				names.remove(dep);
			}
		}

		names
	}

	/// This runs a Kahn's algorithm for toposort.
	///
	/// It is probably horrendously inefficient but it is
	/// only run once at startup so it most likely doesn't matter.
	fn system_toposort(&mut self) -> Vec<Box<AbstractBuilder>> {
		let mut result = vec![];

		while !self.sysmap.is_empty() {
			let roots = self.find_roots().into_iter().collect::<Vec<_>>();

			if roots.is_empty() {
				panic!("Cycle detected within dependencies");
			}

			for root in roots {
				if let Some(sys) = self.sysmap.remove(root) {
					result.push(sys);
				} else {
					panic!("Cycle detected with system {} as part of it", root);
				}
			}
		}

		result
	}
}

use specs::*;

use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::mem;

use crate::dispatch::sysbuilder::*;
use crate::dispatch::sysinfo::*;

use crate::utils::{EventHandler, EventHandlerTypeProvider};

pub struct Builder<'a, 'b> {
	builder: DispatcherBuilder<'a, 'b>,
	sysmap: HashMap<&'static str, Box<dyn AbstractBuilder>>,
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
		use crate::utils::EventHandlerWrapper;
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

		let mut count: usize = 0;
		for sys in &systems {
			debug!(
				target: "airmash:builder",
				"{:03} Added system to builder: {name}",
				count,
				name = sys.name(),
			);
			count += 1;

			for dep in sys.deps() {
				debug!(
					target: "airmash:builder",
					"{:03}   With dependency: {}", 
					count, 
					dep
				);
				count += 1;
			}
		}

		self.builder = systems
			.into_iter()
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

fn make_hashset<T>(val: T) -> HashSet<T> 
where
	T: std::hash::Hash + Eq
{
	let mut set = HashSet::default();
	set.insert(val);
	set
}

#[derive(Default)]
struct Graph<'a> {
	vertices: HashSet<&'a str>,
	incoming: HashMap<&'a str, HashSet<&'a str>>,
	outgoing: HashMap<&'a str, HashSet<&'a str>>,
	roots: HashSet<&'a str>,
}

impl<'a> Graph<'a> {
	pub fn insert_vertex(&mut self, vert: &'a str) -> bool {
		self.roots.insert(vert);
		self.vertices.insert(vert)
	}

	fn insert_incoming(&mut self, target: &'a str, source: &'a str) {
		if let Some(x) = self.incoming.get_mut(target) {
			x.insert(source);
		} else {
			self.incoming.insert(target, make_hashset(source));
		}
	}
	fn insert_outgoing(&mut self, target: &'a str, source: &'a str) {
		if let Some(x) = self.outgoing.get_mut(source) {
			x.insert(target);
		} else {
			self.outgoing.insert(source, make_hashset(target));
		}
	}

	pub fn insert_edge(&mut self, target: &'a str, source: &'a str) {
		assert!(self.vertices.contains(target));
		assert!(self.vertices.contains(source));
		
		self.insert_incoming(target, source);
		self.insert_outgoing(target, source);

		self.roots.remove(target);
	}

	fn remove_edge(&mut self, target: &'a str, source: &'a str) {
		let in_empty = if let Some(x) = self.incoming.get_mut(target) {
			x.remove(source);
			x.is_empty()
		} else { false };
		let out_empty = if let Some(x) = self.outgoing.get_mut(source) {
			x.remove(target);
			x.is_empty()
		} else { false };

		if in_empty {
			self.incoming.remove(target);
		}
		if out_empty {
			self.outgoing.remove(source);
		}
	}
	fn has_incoming(&self, target: &'a str) -> bool {
		self.incoming.get(target).is_some()
	}

	/// Toposort using Kahn's algorithm
	pub fn toposort(mut self) -> Vec<&'a str> {
		use std::collections::VecDeque;

		let mut res = vec![];
		let mut roots = self.roots
			.iter()
			.cloned()
			.collect::<VecDeque<_>>();

		while let Some(node) = roots.pop_front() {
			res.push(node);

			if let Some(sources) = self.outgoing.get(node).cloned() {
				for m in sources {
					self.remove_edge(m, node);

					if !self.has_incoming(m) {
						roots.push_back(m);
					}
				}
			}
		}

		// If we find a cycle, print a nice(ish) error
		// message to help with debugging it.
		if res.len() != self.vertices.len() {
			let res = res.into_iter().collect::<HashSet<_>>();
			
			error!("Cycle found in dependancy graph containing the following systems:");
			for name in self.vertices.difference(&res) {
				error!("  {}", name);
			}

			panic!("Cycle found in dependency graph!");
		}

		res.reverse();

		res
	}
}

// This impl is related to finding a toposort of
// the systems so that they can be registered in
// the correct order.
impl<'a, 'b> Builder<'a, 'b> {
	fn system_toposort(&mut self) -> Vec<Box<dyn AbstractBuilder>> {
		let mut graph = Graph::default();

		for (name, _) in self.sysmap.iter() {
			graph.insert_vertex(name);
		}

		for (name, sys) in self.sysmap.iter() {
			let deps = sys.deps();

			for dep in deps {
				graph.insert_edge(dep, name);
			}
		}

		let sorted = graph.toposort();
		let mut result = vec![];

		for name in sorted {
			let sys = self.sysmap.remove(name);

			if let Some(sys) = sys {
				result.push(sys);
			} else {
				error!("Unknown system {}", name);
				error!("Do you have a dependency on a system that wasn't added?");
				panic!("Unknown system.");
			}
		}

		result
	}
}

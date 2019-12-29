use super::anyvec::AnyVec;
use super::system::*;
use super::vtable::DynSystemVTable;
use super::World;

use std::any::TypeId;
use std::fmt;

pub struct Builder<'world> {
    world: &'world mut World,
    builders: Vec<Box<dyn DynSystemBuilder>>,
}

impl<'world> Builder<'world> {
    pub fn new(world: &'world mut World) -> Self {
        Self {
            world,
            builders: vec![],
        }
    }

    pub fn world(&mut self) -> &mut World {
        &mut *self.world
    }

    pub fn with_builder<S: SystemBuilder + 'static>(&mut self, system: S) -> &mut Self {
        self.builders
            .push(Box::new(DynSystemBuilderImpl::new(system)));
        self
    }

    pub fn with<S: Default + SystemBuilder + 'static>(&mut self) -> &mut Self {
        self.with_builder(S::default())
    }

    /// Call the passed in function with self and
	/// return whatever the function returns.
	///
	/// This is meant as an ease-of-use wrapper
	/// for `register` style functions.
    pub fn with_registrar<F>(&mut self, fun: F) -> &mut Self
    where
        F: FnOnce(&mut Self)
    {
        fun(self);
        self
    }

    pub fn build(mut self) -> Result<Dispatcher, CycleError> {
        use petgraph::{algo::toposort, Directed, Graph};
        use std::collections::hash_map::RandomState;
        use std::collections::HashMap;

        let mut writes: HashMap<_, Vec<_>> = HashMap::default();
        let mut reads: HashMap<_, Vec<_>> = HashMap::default();
        let mut graph: Graph<_, _, Directed> = Graph::new();

        let mut tmp = Vec::new();
        let mut nodes: HashMap<_, _, RandomState> = HashMap::default();

        // Insert all nodes and add all writes/reads
        for (idx, builder) in self.builders.iter().enumerate() {
            let type_id = builder.type_id();

            builder.reads(&mut tmp);
            for id in tmp.drain(..) {
                reads.entry(id).or_insert_with(Vec::new).push(type_id);
            }

            builder.writes(&mut tmp);
            for id in tmp.drain(..) {
                writes.entry(id).or_insert_with(Vec::new).push(type_id);
            }

            let node = graph.add_node(idx);
            nodes.insert(type_id, node);
        }

        // Add all dependency edges
        for builder in self.builders.iter() {
            let node = nodes[&builder.type_id()];

            builder.dependencies(&mut tmp);
            for dep in tmp.drain(..) {
                let depnode = nodes[&dep];

                graph.add_edge(node, depnode, 1);
            }
        }

        // Add read/write edges
        for (event, readers) in reads {
            let writers = match writes.remove(&event) {
                Some(writes) => writes,
                None => continue,
            };

            for writer in writers.iter() {
                for reader in readers.iter() {
                    graph.add_edge(nodes[reader], nodes[writer], 1);
                }
            }
        }

        let order = match toposort(&graph, None) {
            Ok(order) => order,
            Err(cycle) => {
                let builder = &self.builders[graph[cycle.node_id()]];
                let system = builder.type_name();
                return Err(CycleError { system });
            }
        };

        let mut systems = AnyVec::new();

        for index in order {
            let builder_idx = graph[index];
            let builder = &mut self.builders[builder_idx];
            builder.build(self.world, &mut systems);
        }

        Ok(Dispatcher { systems })
    }
}

pub struct Dispatcher {
    systems: AnyVec<DynSystemVTable>,
}

impl Dispatcher {
    pub fn builder<'world>(world: &'world mut World) -> Builder<'world> {
        Builder::new(world)
    }

    pub fn dispatch_all(&mut self, world: &mut World) {
        for system in self.systems.iter_mut() {
            system.run(world);
        }
    }
}

struct SystemList<'a>(&'a Dispatcher);

impl fmt::Debug for SystemList<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut list = fmt.debug_list();

        for system in self.0.systems.iter() {
            list.entry(&system.type_name());
        }

        list.finish()
    }
}

impl fmt::Debug for Dispatcher {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Dispatcher")
            .field("systems", &SystemList(self))
            .finish()
    }
}

#[derive(Debug)]
pub struct CycleError {
    system: &'static str,
}

impl fmt::Display for CycleError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            fmt,
            "Encountered a cycle containing the following system: {}",
            self.system
        )
    }
}

impl std::error::Error for CycleError {}

/// Dynamic system implementation
pub(super) trait DynSystemBuilder {
    /// User-specified system dependencies
    fn dependencies(&self, deps: &mut Vec<TypeId>);

    /// Channels that this system writes to
    fn reads(&self, reads: &mut Vec<TypeId>);
    /// Channels that this system reads from
    fn writes(&self, writes: &mut Vec<TypeId>);

    fn type_id(&self) -> TypeId;
    fn type_name(&self) -> &'static str;

    /// Build the system
    fn build(&mut self, world: &mut World, dest: &mut AnyVec<DynSystemVTable>);
}

struct DynSystemBuilderImpl<S>(Option<S>);

impl<S> DynSystemBuilderImpl<S> {
    pub fn new(sys: S) -> Self {
        Self(Some(sys))
    }
}

impl<S> DynSystemBuilder for DynSystemBuilderImpl<S>
where
    S: SystemBuilder + 'static,
{
    fn dependencies(&self, deps: &mut Vec<TypeId>) {
        S::Dependencies::dependencies(deps);
    }

    fn reads(&self, reads: &mut Vec<TypeId>) {
        S::Dependencies::reads(reads);
    }
    fn writes(&self, writes: &mut Vec<TypeId>) {
        S::Dependencies::writes(writes);
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<S>()
    }
    fn type_name(&self) -> &'static str {
        std::any::type_name::<S>()
    }

    fn build(&mut self, world: &mut World, dest: &mut AnyVec<DynSystemVTable>) {
        if self.0.is_none() {
            unreachable!("DynSystemBuilder::build called twice!");
        }

        let mut sys = self.0.take().unwrap().build();
        sys.setup(world);

        dest.push(sys);
    }
}

//! Experimental ECS.

mod anymap;
mod anyvec;
mod dispatch;
mod storage;
mod system;
mod vtable;
mod world;
mod systemdata;

// Needed for vtable derive macro
use self::vtable::VTable;
use self::dispatch::DynSystem;

pub use self::storage::{Component, DynStorage, Storage};
pub use self::storage::{VecStorage, NullStorage};
pub use self::world::World;
pub use self::dispatch::Builder;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Entity {
    id: usize,
    gen: usize,
}

impl Entity {
    pub fn new(id: usize, gen: usize) -> Self {
        Self { id, gen }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn gen(&self) -> usize {
        self.gen
    }
}

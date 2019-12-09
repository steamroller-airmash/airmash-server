//! Experimental ECS.

mod anymap;
mod anyvec;
mod dispatch;
mod storage;
mod system;
mod systemdata;
mod vtable;
mod world;

// Needed for vtable derive macro
use self::dispatch::DynSystem;
use self::vtable::VTable;

pub use self::dispatch::Builder;
pub use self::storage::{Component, DynStorage, Storage};
pub use self::storage::{NullStorage, VecStorage};
pub use self::world::World;

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

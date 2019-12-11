//! Experimental ECS.

mod anymap;
mod anyvec;
mod dispatch;
mod error;
mod resource;
mod storage;
mod system;
mod systemdata;
mod vtable;
mod world;

// Needed for vtable derive macro
use self::resource::EntityRes;
use self::vtable::VTable;

pub use self::dispatch::Builder;
pub use self::error::EntityDead;
pub use self::resource::EntityRef;
pub use self::storage::{Component, DynStorage, EntityStorage, EntityStorageMut, Storage};
pub use self::storage::{NullStorage, VecStorage};
pub use self::system::{DynSystem, System, SystemBuilder, SystemData, SystemDeps};
pub use self::systemdata::*;
pub use self::world::World;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Entity {
    id: u32,
    gen: u32,
}

impl Entity {
    pub fn new(id: u32, gen: u32) -> Self {
        Self { id, gen }
    }

    pub fn id(self) -> u32 {
        self.id
    }

    pub fn gen(self) -> u32 {
        self.gen
    }
}

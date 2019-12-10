use crate::ecs::{
    Component, Entity, EntityRef, EntityRes, EntityStorage, Storage, SystemData,
    World, EntityDead,
};

use std::any::TypeId;
use std::cell::Ref;
use std::marker::PhantomData;

/// The entities of this ECS.
/// 
/// This is what you'll need to create, delete, or borrow
/// entities.
pub struct Entities<'a> {
    res: Ref<'a, EntityRes>,
}

impl<'a> Entities<'a> {
	/// Create a new entity. 
	pub fn create(&self) -> Entity {
		self.res.create()
	}
	/// Delete an existing entity. If the has already been
	/// deleted then an error is returned.
	/// 
	/// # Note
	/// Even after deleting an entity it will continue to
	/// be alive until all the references to the entity
	/// are dropped (`EntityRef` instances) and the entity
	/// is garbage collected.
	/// 
	/// If you want to revive an already-deleted entity
	/// that hasn't been garbage collected yet you'll
	/// want to use `revive`.
	pub fn delete(&self, entity: Entity) -> Result<(), EntityDead> {
		self.res.delete(entity)
	}

	/// Revive an existing entity that has been deleted but
	/// hasn't yet been cleaned up by GC. This usually means
	/// that either
	/// 
	/// 1. We're still within the same frame as when this entity
	///    was originally deleted.
	/// 2. There are `EntityRef` references keeping this entity
	///    alive.
	/// 
	/// In either case this marks the entity as being alive again.
	/// If the entity is already alive then this is a no-op.
	pub fn revive(&self, entity: Entity) -> Result<(), EntityDead> {
		self.res.revive(entity)
	}

	/// Create an entity directly from an id. There are no guarantees
	/// that the resulting entity is alive.
	pub fn forge(&self, id: u32) -> Entity {
		self.res.forge(id)
	}

	/// Create an entity and return a builder that makes it easy
	/// to insert a variety of components into storages.
	pub fn build(&self) -> EntityBuilder<'a> {
		EntityBuilder::new(self.create(), self)
	}

	/// Check whether an entity is accessible. This includes
	/// both live and zombie entities.
    pub fn is_accesssible(&self, entity: Entity) -> bool {
        self.res.is_accessible(entity)
	}

	/// Check whether an entity is alive. A live entity
	/// is one that was created but hasn't yet been deleted.
	pub fn is_alive(&self, entity: Entity) -> bool {
		self.res.is_alive(entity)
	}
	
	/// Check whether an entity has been deleted but not GCed yet.
	/// 
	/// Note that all zombie are considered to be alive.
	pub fn is_zombie(&self, entity: Entity) -> bool {
		self.res.is_zombie(entity)
	}

	/// Create a reference to the entity that will keep
	/// it alive for as long as the reference remains alive.
	/// 
	/// This is useful for ensuring that an entity will not
	/// be deleted while it's still needed.
    pub fn borrow(&self, entity: Entity) -> Result<EntityRef, EntityDead> {
		self.res.borrow(entity)
	}
}

impl<'a> SystemData<'a> for Entities<'a> {
    fn fetch(world: &'a World) -> Self {
        let res = world
            .fetch_resource()
            .expect("EntitiesRes has not been registered!");

        Self { res }
    }

    fn setup(world: &mut World) {
        world.register_resource_lazy(|| EntityRes::new());
    }

    fn reads(_: &mut Vec<TypeId>) {}
    fn writes(_: &mut Vec<TypeId>) {}
}

/// Helper for creating an entity with a bunch of components.
pub struct EntityBuilder<'a> {
	entity: Entity,
	_marker: PhantomData<&'a ()>
}

impl<'a> EntityBuilder<'a> {
	fn new(entity: Entity, _: &Entities<'a>) -> Self {
		Self {
			entity,
			_marker: PhantomData
		}
	}

	/// Insert a component into the storage for this entity.
    pub fn with<T, S>(&self, storage: &mut S, val: T) -> &Self
    where
        T: Component<Storage = S::Storage>,
        S: EntityStorage<T>,
    {
        storage.storage_mut().insert(self.entity.id(), val);
        self
    }

	/// Finish building and get the entity.
    pub fn build(self) -> Entity {
        self.entity
    }
}

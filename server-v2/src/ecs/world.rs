use super::anymap::AnyMap;
use super::vtable::{DynStorageVTable, VTable};
use super::{Component, DynStorage, Entities, Entity, EntityRes, SystemData, WriteStorage};

use std::any::TypeId;
use std::cell::{Ref, RefCell, RefMut};

use hibitset::BitSetLike;

pub struct World {
    storages: AnyMap<DynStorageVTable>,
    resources: AnyMap<()>,
}

impl World {
    pub fn new() -> Self {
        let mut me = Self {
            storages: AnyMap::new(),
            resources: AnyMap::new(),
        };

        // Register some core resources that are essential for the
        // base functions of the ECS.
        me.register_resource(EntityRes::new());

        me
    }

    pub fn register_storage<T: DynStorage + 'static>(&mut self, val: T) -> &mut T {
        let cell = RefCell::new(val);
        let vtable = DynStorageVTable::from_existing(&cell);
        self.storages.insert(cell, vtable).get_mut()
    }

    pub fn register_storage_lazy<T, F>(&mut self, func: F) -> &mut T
    where
        T: DynStorage + 'static,
        F: FnOnce() -> T,
    {
        if !self.storages.contains(TypeId::of::<RefCell<T>>()) {
            self.register_storage(func())
        } else {
            self.storages
                .get_mut::<RefCell<T>>()
                .map(|x| x.0)
                .expect("Existing storage actually not present")
                .get_mut()
        }
    }

    pub fn register_resource<T: 'static>(&mut self, val: T) -> &mut T {
        self.resources.insert(RefCell::new(val), ()).get_mut()
    }

    pub fn register_resource_lazy<T, F>(&mut self, func: F) -> &mut T
    where
        T: 'static,
        F: FnOnce() -> T,
    {
        if !self.resources.contains(TypeId::of::<RefCell<T>>()) {
            self.register_resource(func())
        } else {
            self.resources
                .get_mut::<RefCell<T>>()
                .map(|x| x.0.get_mut())
                .expect("Existing resource actually not present")
        }
    }

    pub fn fetch_storage<T: 'static>(&self) -> Ref<T> {
        self.storages
            .get::<RefCell<T>>()
            .map(|(cell, _)| cell.borrow())
            .unwrap_or_else(|| panic!("Unable to fetch storage `{}`", std::any::type_name::<T>()))
    }
    pub fn fetch_storage_mut<T: 'static>(&self) -> RefMut<T> {
        self.storages
            .get::<RefCell<T>>()
            .map(|(cell, _)| cell.borrow_mut())
            .unwrap_or_else(|| panic!("Unable to fetch storage `{}`", std::any::type_name::<T>()))
    }

    pub fn fetch_resource<T: 'static>(&self) -> Ref<T> {
        self.resources
            .get::<RefCell<T>>()
            .map(|(cell, _)| cell.borrow())
            .unwrap_or_else(|| panic!("Unable to fetch resource `{}`", std::any::type_name::<T>()))
    }
    pub fn fetch_resource_mut<T: 'static>(&self) -> RefMut<T> {
        self.resources
            .get::<RefCell<T>>()
            .map(|(cell, _)| cell.borrow_mut())
            .unwrap_or_else(|| panic!("Unable to fetch resource `{}`", std::any::type_name::<T>()))
    }

    pub fn remove_resource<T: 'static>(&mut self) -> bool {
        self.resources.remove::<T>()
    }

    pub fn iter_storages(&self) -> impl Iterator<Item = &(dyn DynStorage + 'static)> {
        self.storages
            .iter()
            .map(|(_, storage, meta)| unsafe { meta.rebuild(storage) })
    }
    pub fn iter_storages_mut(&mut self) -> impl Iterator<Item = &mut (dyn DynStorage + 'static)> {
        self.storages
            .iter_mut()
            .map(|(_, storage, meta)| unsafe { meta.rebuild_mut(storage) })
    }

    /// Fetch a type implementing `SystemData` from the world.
    pub fn system_data<'a, S: SystemData<'a>>(&'a self) -> S {
        S::fetch(self)
    }

    /// Create a new entity directly using the world.
    pub fn create_entity<'a>(&'a self) -> WorldEntityBuilder<'a> {
        WorldEntityBuilder::new(self)
    }

    pub fn maintain(&mut self) {
        // TODO: Are there other maintainance tasks that need to be done?
        //  - e.g. lazy tasks?

        self._maintain_gc()
    }
}

// Maintainance-related utilities
impl World {
    /// Clean up any dead entities and drop their components
    fn _maintain_gc(&mut self) {
        let removed = self.fetch_resource_mut::<EntityRes>().gc();

        if removed.is_empty() {
            return;
        }

        for storage in self.iter_storages_mut() {
            storage.remove_all(&removed);
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

pub struct WorldEntityBuilder<'world> {
    world: &'world World,
    entity: Entity,
}

impl<'w> WorldEntityBuilder<'w> {
    fn new(world: &'w World) -> Self {
        let entities: Entities = world.system_data();
        let entity = entities.create();

        Self { world, entity }
    }

    pub fn with<C: Component + 'static>(self, component: C) -> Self {
        let mut storage: WriteStorage<C> = self.world.system_data();
        storage
            .insert(self.entity, component)
            .expect("Entity builder created with dead entity");

        self
    }

    pub fn build(self) -> Entity {
        self.entity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_entityres() {
        let world = World::new();

        let _ = world.fetch_resource::<EntityRes>();
    }
}

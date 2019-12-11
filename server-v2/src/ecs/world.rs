use super::anymap::AnyMap;
use super::vtable::{DynStorageVTable, VTable};
use super::{DynStorage, EntityRes};

use std::any::TypeId;
use std::cell::{Ref, RefCell, RefMut};

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

    pub fn register_storage<T: DynStorage + 'static>(&mut self, val: T) {
        self.resources.insert(RefCell::new(val), ());
    }

    pub fn register_storage_lazy<T, F>(&mut self, func: F)
    where
        T: DynStorage + 'static,
        F: FnOnce() -> T,
    {
        if !self.storages.contains(TypeId::of::<RefCell<T>>()) {
            let val = func();
            let vtable = DynStorageVTable::from_existing(&val);
            self.storages.insert(RefCell::new(val), vtable);
        }
    }

    pub fn register_resource<T: 'static>(&mut self, val: T) {
        self.resources.insert(RefCell::new(val), ());
    }

    pub fn register_resource_lazy<T, F>(&mut self, func: F)
    where
        T: 'static,
        F: FnOnce() -> T,
    {
        if !self.resources.contains(TypeId::of::<RefCell<T>>()) {
            self.resources.insert(RefCell::new(func()), ());
        }
    }

    pub fn fetch_storage<T: 'static>(&self) -> Option<Ref<T>> {
        self.storages
            .get::<RefCell<T>>()
            .map(|(cell, _)| cell.borrow())
    }
    pub fn fetch_storage_mut<T: 'static>(&self) -> Option<RefMut<T>> {
        self.storages
            .get::<RefCell<T>>()
            .map(|(cell, _)| cell.borrow_mut())
    }

    pub fn fetch_resource<T: 'static>(&self) -> Option<Ref<T>> {
        self.resources
            .get::<RefCell<T>>()
            .map(|(cell, _)| cell.borrow())
    }
    pub fn fetch_resource_mut<T: 'static>(&self) -> Option<RefMut<T>> {
        self.resources
            .get::<RefCell<T>>()
            .map(|(cell, _)| cell.borrow_mut())
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
        let removed = match self.fetch_resource_mut::<EntityRes>() {
            Some(mut res) => res.gc(),
            None => unreachable!("EntityRes has not been registered"),
        };

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

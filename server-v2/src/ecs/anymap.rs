#![allow(dead_code)]

use std::any::{Any, TypeId};
use std::collections::HashMap;

struct MapItem<M, V: Any + ?Sized> {
    meta: M,
    value: V,
}

impl<M> MapItem<M, dyn Any> {
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.value.downcast_ref()
    }

    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.value.downcast_mut()
    }
}

pub struct AnyMap<M> {
    map: HashMap<TypeId, Box<MapItem<M, dyn Any>>>,
}

impl<M> AnyMap<M> {
    pub fn new() -> Self {
        Self {
            map: HashMap::default(),
        }
    }

    pub fn get<T: 'static>(&self) -> Option<(&T, &M)> {
        self.map
            .get(&TypeId::of::<T>())
            .map(|val| (&val.value, &val.meta))
            .and_then(|(x, m)| Some((Any::downcast_ref(x)?, m)))
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<(&mut T, &M)> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .map(|val| (&mut val.value, &val.meta))
            .and_then(|(x, m)| Some((Any::downcast_mut(x)?, m)))
    }

    pub fn insert<T: 'static>(&mut self, value: T, meta: M) -> &mut T {
        let mut entry = self.map.entry(TypeId::of::<T>())
            .insert(Box::new(MapItem { meta, value }));
        let entry = entry.get_mut().downcast_mut().unwrap();

        // TODO: Is this ok?
        unsafe { &mut *(entry as *mut _) }
    }

    pub fn remove<T: 'static>(&mut self) -> bool {
        self.map.remove(&TypeId::of::<T>()).is_some()
    }

    pub fn contains(&self, id: TypeId) -> bool {
        self.map.contains_key(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (TypeId, &dyn Any, &M)> {
        self.map
            .iter()
            .map(|(id, item)| (*id, &item.value, &item.meta))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (TypeId, &mut dyn Any, &M)> {
        self.map
            .iter_mut()
            .map(|(id, item)| (*id, &mut item.value, &item.meta))
    }
}

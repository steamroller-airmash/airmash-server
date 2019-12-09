#![allow(dead_code)]

use std::any::{Any, TypeId};
use std::collections::HashMap;

struct MapItem<M, V: Any + ?Sized> {
    meta: M,
    value: V,
}

pub struct AnyMap<M> {
    map: HashMap<TypeId, Box<MapItem<M, dyn Any>>>,
}

impl<M> AnyMap<M> {
    pub fn new() -> Self {
        Self {
            map: HashMap::default()
        }
    }

    pub fn get<T: Any>(&self) -> Option<(&T, &M)> {
        self.map.get(&TypeId::of::<T>())
            .map(|val| (&val.value, &val.meta))
            .and_then(|(x, m)| Some((Any::downcast_ref(x)?, m)))
    }

    pub fn get_mut<T: Any>(&mut self) -> Option<(&mut T, &M)> {
        self.map.get_mut(&TypeId::of::<T>())
            .map(|val| (&mut val.value, &val.meta))
            .and_then(|(x, m)| Some((Any::downcast_mut(x)?, m)))
    }

    pub fn insert<T: Any>(&mut self, value: T, meta: M) {
        self.map.insert(TypeId::of::<T>(), Box::new(MapItem {
            meta,
            value
        }));
    }

    pub fn contains(&self, id: TypeId) -> bool {
        self.map.contains_key(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (TypeId, &dyn Any, &M)> {
        self.map.iter()
            .map(|(id, item)| (*id, &item.value, &item.meta))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (TypeId, &mut dyn Any, &M)> {
        self.map.iter_mut()
            .map(|(id, item)| (*id, &mut item.value, &item.meta))
    }
}

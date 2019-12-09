use super::{DynStorage, Storage};
use hibitset::BitSet;
use std::collections::HashMap;

pub struct HashMapStorage<T> {
    bitset: BitSet,
    // TODO: Use a better hasher?
    map: HashMap<u32, T>,
}

impl<T> Default for HashMapStorage<T> {
    fn default() -> Self {
        Self {
            bitset: BitSet::new(),
            map: HashMap::default(),
        }
    }
}

impl<T> DynStorage for HashMapStorage<T> {
    fn remove(&mut self, ent: u32) {
        <Self as Storage<T>>::remove(self, ent);
    }

    fn remove_all(&mut self, mask: &BitSet) {
        <Self as Storage<T>>::remove_all(self, mask);
    }
}

impl<T> Storage<T> for HashMapStorage<T> {
    fn mask(&self) -> &BitSet {
        &self.bitset
    }

    fn insert(&mut self, ent: u32, val: T) -> Option<T> {
        self.bitset.add(ent);
        self.map.insert(ent, val)
    }
    fn remove(&mut self, ent: u32) -> Option<T> {
        self.bitset.remove(ent);
        self.map.remove(&ent)
    }

    fn remove_all(&mut self, bits: &BitSet) {
        let bitand = bits & self.bitset.clone();
        self.bitset &= &!bits;

        for idx in bitand {
            self.map.remove(&idx);
        }
    }

    fn get(&self, ent: u32) -> Option<&T> {
        self.map.get(&ent)
    }
    fn get_mut(&mut self, ent: u32) -> Option<&mut T> {
        self.map.get_mut(&ent)
    }

    unsafe fn get_unchecked(&self, ent: u32) -> &T {
        match self.map.get(&ent) {
            Some(x) => x,
            None => std::hint::unreachable_unchecked(),
        }
    }
    unsafe fn get_mut_unchecked(&mut self, ent: u32) -> &mut T {
        match self.map.get_mut(&ent) {
            Some(x) => x,
            None => std::hint::unreachable_unchecked(),
        }
    }
}

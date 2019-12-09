use super::{DynStorage, Storage};

use hibitset::BitSet;

use std::mem;

pub struct NullStorage<T>
where
    T: Default,
{
    marker: T,
    bitset: BitSet,
}

impl<T: Default> NullStorage<T> {
    pub fn new() -> Self {
        assert!(
            mem::size_of::<T>() == 0,
            "Null storage can only be used with zero-sized types!"
        );

        Self {
            marker: T::default(),
            bitset: BitSet::new(),
        }
    }
}

impl<T: Default> DynStorage for NullStorage<T> {
    fn remove(&mut self, ent: u32) {
        <Self as Storage<T>>::remove(self, ent);
    }

    fn remove_all(&mut self, mask: &BitSet) {
        <Self as Storage<T>>::remove_all(self, mask);
    }
}

impl<T: Default> Storage<T> for NullStorage<T> {
    fn mask(&self) -> &BitSet {
        &self.bitset
    }

    fn insert(&mut self, ent: u32, _: T) -> Option<T> {
        if self.bitset.add(ent) {
            Some(T::default())
        } else {
            None
        }
    }
    fn remove(&mut self, ent: u32) -> Option<T> {
        if self.bitset.remove(ent) {
            Some(T::default())
        } else {
            None
        }
    }

    fn remove_all(&mut self, bits: &BitSet) {
        self.bitset &= &!bits;
    }

    fn get(&self, ent: u32) -> Option<&T> {
        if self.bitset.contains(ent) {
            Some(&self.marker)
        } else {
            None
        }
    }
    fn get_mut(&mut self, ent: u32) -> Option<&mut T> {
        if self.bitset.contains(ent) {
            Some(&mut self.marker)
        } else {
            None
        }
    }

    unsafe fn get_unchecked(&self, _: u32) -> &T {
        &self.marker
    }
    unsafe fn get_mut_unchecked(&mut self, _: u32) -> &mut T {
        &mut self.marker
    }
}

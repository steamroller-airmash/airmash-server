use super::{DynStorage, Storage, VecStorage};
use fxhash::FxHashMap;
use hibitset::{BitSet, BitSetLike};

use std::mem;

pub struct DenseVecStorage<T> {
    backing: Vec<T>,
    indices: VecStorage<usize>,
    reverse: FxHashMap<usize, u32>,
}

impl<T> DenseVecStorage<T> {
    pub fn new() -> Self {
        Self {
            backing: Vec::new(),
            indices: VecStorage::new(),
            reverse: FxHashMap::default(),
        }
    }
}

impl<T> DynStorage for DenseVecStorage<T> {
    fn mask(&self) -> &BitSet {
        <Self as Storage<_>>::mask(self)
    }

    fn remove(&mut self, ent: u32) {
        <Self as Storage<T>>::remove(self, ent);
    }

    fn remove_all(&mut self, mask: &BitSet) {
        <Self as Storage<T>>::remove_all(self, mask);
    }
}

impl<T> Storage<T> for DenseVecStorage<T> {
    fn mask(&self) -> &BitSet {
        Storage::mask(&self.indices)
    }

    fn insert(&mut self, ent: u32, val: T) -> Option<T> {
        if let Some(&index) = self.indices.get(ent) {
            return Some(mem::replace(
                unsafe { self.backing.get_unchecked_mut(index) },
                val,
            ));
        }

        let index = self.backing.len();
        self.backing.push(val);
        self.indices.insert(ent, index);
        self.reverse.insert(index, ent);

        None
    }

    fn remove(&mut self, ent: u32) -> Option<T> {
        let index = Storage::remove(&mut self.indices, ent)?;
        let value = self.backing.swap_remove(index);
        let prev = self.backing.len();

        let prev_ent = match self.reverse.remove(&prev) {
            Some(prev) => prev,
            None => unreachable!(),
        };

        if prev_ent != ent {
            self.reverse.insert(index, prev_ent).unwrap();
        }

        Some(value)
    }

    fn remove_all<B: BitSetLike>(&mut self, bits: B) {
        for index in bits.iter() {
            Storage::remove(self, index);
        }
    }

    fn get(&self, ent: u32) -> Option<&T> {
		let index = *self.indices.get(ent)?;
		
		debug_assert!(index < self.backing.len());
        Some(unsafe { self.backing.get_unchecked(index) })
    }

    fn get_mut(&mut self, ent: u32) -> Option<&mut T> {
        let index = *self.indices.get(ent)?;

		debug_assert!(index < self.backing.len());
        Some(unsafe { self.backing.get_unchecked_mut(index) })
    }

    unsafe fn get_unchecked(&self, ent: u32) -> &T {
        let index = *self.indices.get_unchecked(ent);
        self.backing.get_unchecked(index)
    }

    unsafe fn get_mut_unchecked(&mut self, ent: u32) -> &mut T {
        let index = *self.indices.get_unchecked(ent);
        self.backing.get_unchecked_mut(index)
    }
}

impl<T> Default for DenseVecStorage<T> {
    fn default() -> Self {
        Self::new()
    }
}

use super::{DynStorage, Storage};
use hibitset::{BitSet, BitSetLike, BitSetNot};
use std::mem::MaybeUninit;

pub struct VecStorage<T> {
    backing: Vec<MaybeUninit<T>>,
    bitset: BitSet,
}

impl<T> VecStorage<T> {
    pub fn new() -> Self {
        Self {
            backing: Vec::new(),
            bitset: BitSet::new(),
        }
    }
}

impl<T> DynStorage for VecStorage<T> {
    fn mask(&mut self) -> &BitSet {
        <Self as Storage<_>>::mask(self)
    }

    fn remove(&mut self, ent: u32) {
        <Self as Storage<T>>::remove(self, ent);
    }

    fn remove_all(&mut self, mask: &BitSet) {
        <Self as Storage<T>>::remove_all(self, mask);
    }
}

impl<T> Storage<T> for VecStorage<T> {
    fn mask(&self) -> &BitSet {
        &self.bitset
    }

    fn insert(&mut self, ent: u32, val: T) -> Option<T> {
        if self.backing.len() <= ent as usize {
            self.backing
                .resize_with(ent as usize + 1, MaybeUninit::uninit);
        }

        let orig = std::mem::replace(&mut self.backing[ent as usize], MaybeUninit::new(val));

        if self.bitset.add(ent as u32) {
            Some(unsafe { orig.assume_init() })
        } else {
            None
        }
    }
    fn remove(&mut self, ent: u32) -> Option<T> {
        if self.backing.len() < ent as usize {
            return None;
        }

        if self.bitset.remove(ent) {
            Some(unsafe {
                std::mem::replace(&mut self.backing[ent as usize], MaybeUninit::uninit())
                    .assume_init()
            })
        } else {
            None
        }
    }

    fn remove_all<B: BitSetLike>(&mut self, bits: B) {
        let bitand = self.bitset.clone() & &bits;
        self.bitset &= &BitSetNot(&bits);

        for idx in bitand {
            unsafe {
                std::mem::replace(&mut self.backing[idx as usize], MaybeUninit::uninit())
                    .assume_init();
            }
        }
    }

    fn get(&self, ent: u32) -> Option<&T> {
        if self.bitset.contains(ent) {
            Some(unsafe { &*self.backing[ent as usize].as_ptr() })
        } else {
            None
        }
    }
    fn get_mut(&mut self, ent: u32) -> Option<&mut T> {
        if self.bitset.contains(ent) {
            Some(unsafe { &mut *self.backing[ent as usize].as_mut_ptr() })
        } else {
            None
        }
    }

    unsafe fn get_unchecked(&self, ent: u32) -> &T {
        &*self.backing.get_unchecked(ent as usize).as_ptr()
    }
    unsafe fn get_mut_unchecked(&mut self, ent: u32) -> &mut T {
        &mut *self.backing.get_unchecked_mut(ent as usize).as_mut_ptr()
    }
}

impl<T> Drop for VecStorage<T> {
    fn drop(&mut self) {
        use std::mem;

        if !mem::needs_drop::<T>() {
            return;
        }

        let bitset = mem::replace(&mut self.bitset, BitSet::new());

        for idx in bitset {
            unsafe {
                mem::replace(
                    self.backing.get_unchecked_mut(idx as usize),
                    MaybeUninit::uninit(),
                )
                .assume_init();
            }
        }
    }
}

impl<T> Default for VecStorage<T> {
    fn default() -> Self {
        Self::new()
    }
}

mod hashmap;
mod null;
mod vec;
// mod custom;

pub use self::hashmap::HashMapStorage;
pub use self::null::NullStorage;
pub use self::vec::VecStorage;

use hibitset::{BitSet, BitSetLike};

/// Dynamic access to storages without knowing its type.
///
/// As a result of this, the operations that are available
/// on this trait are rather limited. Generally, you'll
/// probably want to be using `Storage` instead.
pub trait DynStorage {
    fn mask(&self) -> &BitSet;

    fn remove(&mut self, ent: u32);
    fn remove_all(&mut self, mask: &BitSet);
}

pub trait Storage<T>: DynStorage + Default {
    fn mask(&self) -> &BitSet;

    fn insert(&mut self, ent: u32, val: T) -> Option<T>;
    fn remove(&mut self, ent: u32) -> Option<T>;

    fn remove_all<B: BitSetLike>(&mut self, mask: B);

    fn get(&self, ent: u32) -> Option<&T>;
    fn get_mut(&mut self, ent: u32) -> Option<&mut T>;

    unsafe fn get_unchecked(&self, ent: u32) -> &T {
        match self.get(ent) {
            Some(x) => x,
            None => unreachable!(),
        }
    }
    unsafe fn get_mut_unchecked(&mut self, ent: u32) -> &mut T {
        match self.get_mut(ent) {
            Some(x) => x,
            None => unreachable!(),
        }
    }
}

/// (Somewhat) internal trait used to get at the underlying
/// storage for a wrapper accessor.
pub trait EntityStorage<T> {
    type Storage: Storage<T>;

    fn storage(&self) -> &Self::Storage;
}

/// (Somewhat) internal trait used to get at the underlying
/// storage for a wrapper accessor.
pub trait EntityStorageMut<T>: EntityStorage<T> {
    fn storage_mut(&mut self) -> &mut Self::Storage;
}

pub trait Component: Sized {
    type Storage: Storage<Self>;
}

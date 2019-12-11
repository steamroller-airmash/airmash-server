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
    /// Get a mask which specifies which elements are present
    /// within the storage.
    fn mask(&self) -> &BitSet;

    /// Insert a new element into the storage.
    /// 
    /// If an element exists at that position already,
    /// returns the existing element and inserts the new
    /// one.
    fn insert(&mut self, ent: u32, val: T) -> Option<T>;

    /// Remove an element from the storage.
    /// 
    /// If the element is present in the storage, returns
    /// that element.
    fn remove(&mut self, ent: u32) -> Option<T>;

    /// Removes all elements who are present in the provided
    /// mask.
    fn remove_all<B: BitSetLike>(&mut self, mask: B);

    /// Fetch an element from the storage.
    fn get(&self, ent: u32) -> Option<&T>;

    /// Fetch an element from the storage mutably.
    fn get_mut(&mut self, ent: u32) -> Option<&mut T>;

    /// Fetch an element from the storage without performing bounds
    /// checking or checking to see if the entity is valid.
    /// 
    /// # Safety
    /// It is UB to call this method for an invalid index as
    /// specified by the `mask()` method of this trait.
    /// 
    /// # Implementors
    /// This method must not cause UB if an access is performed
    /// for an index which is currently marked alive in the bitset
    /// returned by `mask()`.
    unsafe fn get_unchecked(&self, ent: u32) -> &T {
        match self.get(ent) {
            Some(x) => x,
            None => unreachable!(),
        }
    }
    
    /// Fetch an element from the storage mutably without performing
    /// bounds checking or checking to see if the entity is valid.
    /// 
    /// # Safety
    /// It is UB to call this method for an invalid index as
    /// specified by the `mask()` method of this trait.
    /// 
    /// # Implementors
    /// This method must not cause UB if an access is performed
    /// for an index which is currently marked alive in the bitset
    /// returned by `mask()`.
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

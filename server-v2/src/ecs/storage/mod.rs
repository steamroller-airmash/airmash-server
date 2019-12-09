mod hashmap;
mod null;
mod vec;

pub use self::hashmap::HashMapStorage;
pub use self::null::NullStorage;
pub use self::vec::VecStorage;

use hibitset::BitSet;

pub trait DynStorage {
    fn remove(&mut self, ent: u32);
    fn remove_all(&mut self, mask: &BitSet);
}

pub trait Storage<T>: DynStorage {
    fn mask(&self) -> &BitSet;

    fn insert(&mut self, ent: u32, val: T) -> Option<T>;
    fn remove(&mut self, ent: u32) -> Option<T>;

    fn remove_all(&mut self, mask: &BitSet);

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

pub trait Component: Sized {
    type Storage: Storage<Self>;
}

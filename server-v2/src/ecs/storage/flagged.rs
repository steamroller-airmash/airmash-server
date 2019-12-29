use super::{DynStorage, Storage};
use hibitset::{BitSet, BitSetLike};
use shrev::EventChannel;

use std::marker::PhantomData;

pub enum ComponentEvent {
    Inserted(u32),
    Modified(u32),
    Removed(u32),
}

pub struct FlaggedStorage<T, S: Storage<T>> {
    storage: S,
    channel: EventChannel<ComponentEvent>,
    _marker: PhantomData<T>,
}

impl<T, S> Storage<T> for FlaggedStorage<T, S>
where
    S: Storage<T>,
{
    fn mask(&self) -> &BitSet {
        Storage::mask(&self.storage)
    }

    fn insert(&mut self, ent: u32, val: T) -> Option<T> {
        self.channel.single_write(ComponentEvent::Inserted(ent));
        self.storage.insert(ent, val)
    }

    fn remove(&mut self, ent: u32) -> Option<T> {
        self.channel.single_write(ComponentEvent::Removed(ent));
        Storage::remove(&mut self.storage, ent)
    }

    fn remove_all<B: BitSetLike>(&mut self, mask: B) {
        let removed = Storage::mask(&self.storage) & &mask;
        for event in removed.iter().map(ComponentEvent::Removed) {
			self.channel.single_write(event);
		}

        Storage::remove_all(&mut self.storage, mask)
    }

    fn get(&self, ent: u32) -> Option<&T> {
        self.storage.get(ent)
    }

    fn get_mut(&mut self, ent: u32) -> Option<&mut T> {
        self.storage.get_mut(ent)
    }

    unsafe fn get_unchecked(&self, ent: u32) -> &T {
        self.storage.get_unchecked(ent)
    }

    unsafe fn get_mut_unchecked(&mut self, ent: u32) -> &mut T {
        self.storage.get_mut_unchecked(ent)
    }
}

impl<T, S> DynStorage for FlaggedStorage<T, S>
where
    S: Storage<T>,
{
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

impl<T, S> Default for FlaggedStorage<T, S>
where
    S: Storage<T> + Default,
{
    fn default() -> Self {
        Self {
            storage: S::default(),
            channel: EventChannel::default(),
            _marker: PhantomData,
        }
    }
}

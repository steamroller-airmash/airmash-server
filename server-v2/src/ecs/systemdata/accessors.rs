
pub use crate::ecs::{
    Component, Entity, EntityDead, EntityStorage, EntityStorageMut, Storage, SystemData, World,
};
pub use super::Entities;

use hibitset::BitSet;
use shrev::EventChannel;

use std::any::TypeId;
use std::cell::{Ref, RefMut};
use std::ops::{Deref, DerefMut};

/// Specializable trait indicating that this resource
/// reads or writes a type for the purposes of system
/// scheduling.
/// 
/// This is used to implement the `reads` function for
/// `Read` and `ReadExpect` and the `writes` function
/// for `Write` and `WriteExpect`.
/// 
/// Normally you won't have to do anything with this trait,
/// it is already implemented for channels which are the
/// main use-case. However, if you have a custom type
/// which needs to specify it's own reads and writes
/// dependencies
pub trait SpecializedResource {
	fn reads(types: &mut Vec<TypeId>);
	fn writes(types: &mut Vec<TypeId>);
}

impl<T> SpecializedResource for T {
	default fn reads(_: &mut Vec<TypeId>) {}
	default fn writes(_: &mut Vec<TypeId>) {}
}

/// Namespace type to ensure that channels don't conflict
/// with other custom dependencies.
/// 
/// This type is never used and should not be exposed
/// publicly.
struct EventChannelNamespace<C> {
	_marker: std::marker::PhantomData<C>
}

impl<C: 'static> SpecializedResource for EventChannel<C> {
	fn reads(types: &mut Vec<TypeId>) {
		types.push(TypeId::of::<EventChannelNamespace<C>>())
	}

	fn writes(types: &mut Vec<TypeId>) {
		types.push(TypeId::of::<EventChannelNamespace<C>>())
	}
}

/// Fetch a resource immutably.
///
/// This will panic if the resource doesn't exist but it
/// will create the resource during setup so that doesn't
/// happen.
pub struct Read<'a, R>
where
    R: Default + 'static,
{
    inner: ReadExpect<'a, R>,
}

impl<'a, R: Default + 'static> SystemData<'a> for Read<'a, R> {
    fn fetch(world: &'a World) -> Self {
        Self {
            inner: ReadExpect::fetch(world),
        }
    }

    fn setup(world: &mut World) {
        world.register_resource_lazy(R::default)
    }

    fn reads(types: &mut Vec<TypeId>) {
		<ReadExpect<R> as SystemData>::reads(types)
	}
    fn writes(types: &mut Vec<TypeId>) {
		<ReadExpect<R> as SystemData>::reads(types)
	}
}

impl<R: Default + 'static> Deref for Read<'_, R> {
    type Target = R;

    fn deref(&self) -> &R {
        &*self.inner
    }
}

/// Fetch a resource mutably.
///
/// This will panic if the resource doesn't exist but it
/// will create the resource during setup so that doesn't
/// happen.
pub struct Write<'a, R>
where
    R: Default + 'static,
{
    inner: WriteExpect<'a, R>,
}

impl<'a, R: Default + 'static> SystemData<'a> for Write<'a, R> {
    fn fetch(world: &'a World) -> Self {
        Self {
            inner: WriteExpect::fetch(world),
        }
    }

    fn setup(world: &mut World) {
        world.register_resource_lazy(R::default)
    }

    fn reads(types: &mut Vec<TypeId>) {
		<WriteExpect<R> as SystemData>::reads(types)
	}
    fn writes(types: &mut Vec<TypeId>) {
		<WriteExpect<R> as SystemData>::writes(types)
	}
}

impl<R: Default + 'static> Deref for Write<'_, R> {
    type Target = R;

    fn deref(&self) -> &R {
        &*self.inner
    }
}

impl<R: Default + 'static> DerefMut for Write<'_, R> {
    fn deref_mut(&mut self) -> &mut R {
        &mut *self.inner
    }
}

/// Fetch a resource immutably.
///
/// This will **not** create the resource during setup so if you
/// wish to avoid panics at runtime you will need to register
/// the resource with the world yourself.
pub struct ReadExpect<'a, R>
where
    R: 'static,
{
    res: Ref<'a, R>,
}

impl<'a, R: 'static> SystemData<'a> for ReadExpect<'a, R> {
    fn fetch(world: &'a World) -> Self {
        match world.fetch_resource() {
            Some(res) => Self { res },
            None => panic!(
                "Resource with type '{}' wasn't registered!",
                std::any::type_name::<R>()
            ),
        }
    }

    fn setup(_: &mut World) {}

    fn reads(types: &mut Vec<TypeId>) {
		<R as SpecializedResource>::reads(types)
	}
    fn writes(_: &mut Vec<TypeId>) {}
}

impl<R: 'static> Deref for ReadExpect<'_, R> {
    type Target = R;

    fn deref(&self) -> &R {
        &*self.res
    }
}

/// Fetch a resource mutably.
///
/// This will **not** create the resource during setup so if you
/// wish to avoid panics at runtime you will need to register
/// the resource with the world yourself.
pub struct WriteExpect<'a, R>
where
    R: 'static,
{
    res: RefMut<'a, R>,
}

impl<'a, R: 'static> SystemData<'a> for WriteExpect<'a, R> {
    fn fetch(world: &'a World) -> Self {
        match world.fetch_resource_mut() {
            Some(res) => Self { res },
            None => panic!(
                "Resource with type '{}' wasn't registered!",
                std::any::type_name::<R>()
            ),
        }
    }

	fn setup(_: &mut World) {}
	
    fn reads(_: &mut Vec<TypeId>) {}
    fn writes(types: &mut Vec<TypeId>) {
		<R as SpecializedResource>::writes(types)
	}
}

impl<R: 'static> Deref for WriteExpect<'_, R> {
    type Target = R;

    fn deref(&self) -> &R {
        &*self.res
    }
}

impl<R: Default + 'static> DerefMut for WriteExpect<'_, R> {
    fn deref_mut(&mut self) -> &mut R {
        &mut *self.res
    }
}

/// Fetch a storage mutably.
pub struct ReadStorage<'a, C>
where
    C: Component + 'static,
{
    entities: Entities<'a>,
    storage: Ref<'a, C::Storage>,
}

impl<'a, C: Component + 'static> ReadStorage<'a, C> {
    pub fn mask(&self) -> &BitSet {
        self.storage.mask()
    }

    pub fn get(&self, entity: Entity) -> Option<&C> {
        if !self.entities.is_accessible(entity) {
            return None;
        }

        self.storage.get(entity.id())
    }
}

impl<'a, C: Component + 'static> SystemData<'a> for ReadStorage<'a, C> {
    fn fetch(world: &'a World) -> Self {
        let storage = match world.fetch_storage() {
            Some(storage) => storage,
            None => panic!(
                "Storage for component {} was not registered!",
                std::any::type_name::<C>()
            ),
        };

        Self {
            storage,
            entities: Entities::fetch(world),
        }
    }

    fn setup(world: &mut World) {
        Entities::setup(world);
        world.register_storage_lazy(C::Storage::default);
    }

    fn reads(_: &mut Vec<TypeId>) {}
    fn writes(_: &mut Vec<TypeId>) {}
}

impl<'a, C: Component + 'static> EntityStorage<C> for ReadStorage<'a, C> {
    type Storage = C::Storage;

    fn storage(&self) -> &Self::Storage {
        &*self.storage
    }
}

/// Fetch a storage immutably.
pub struct WriteStorage<'a, C>
where
    C: Component + 'static,
{
    entities: Entities<'a>,
    storage: RefMut<'a, C::Storage>,
}

impl<'a, C: Component + 'static> WriteStorage<'a, C> {
    pub fn mask(&self) -> &BitSet {
        self.storage.mask()
    }

    pub fn insert(&mut self, entity: Entity, val: C) -> Result<Option<C>, EntityDead> {
        if !self.entities.is_accessible(entity) {
            return Err(EntityDead::new(entity));
        }

        Ok(self.storage.insert(entity.id(), val))
    }
    pub fn remove(&mut self, entity: Entity) -> Result<Option<C>, EntityDead> {
        if !self.entities.is_accessible(entity) {
            return Err(EntityDead::new(entity));
        }

        Ok(self.storage.remove(entity.id()))
    }

    pub fn get(&self, entity: Entity) -> Option<&C> {
        if !self.entities.is_accessible(entity) {
            return None;
        }

        self.storage.get(entity.id())
    }
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        if !self.entities.is_accessible(entity) {
            return None;
        }

        self.storage.get_mut(entity.id())
    }
}

impl<'a, C: Component + 'static> SystemData<'a> for WriteStorage<'a, C> {
    fn fetch(world: &'a World) -> Self {
        let storage = match world.fetch_storage_mut() {
            Some(storage) => storage,
            None => panic!(
                "Storage for component {} was not registered!",
                std::any::type_name::<C>()
            ),
        };

        Self {
            storage,
            entities: Entities::fetch(world),
        }
    }

    fn setup(world: &mut World) {
        Entities::setup(world);
        world.register_storage_lazy(C::Storage::default);
    }

    fn reads(_: &mut Vec<TypeId>) {}
    fn writes(_: &mut Vec<TypeId>) {}
}

impl<'a, C: Component + 'static> EntityStorage<C> for WriteStorage<'a, C> {
    type Storage = C::Storage;

    fn storage(&self) -> &Self::Storage {
        &*self.storage
    }
}

impl<'a, C: Component + 'static> EntityStorageMut<C> for WriteStorage<'a, C> {
    fn storage_mut(&mut self) -> &mut Self::Storage {
        &mut *self.storage
    }
}

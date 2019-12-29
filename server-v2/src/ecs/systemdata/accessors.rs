pub use super::Entities;
pub use crate::ecs::{
    Component, Entity, EntityDead, EntityStorage, EntityStorageMut, Storage, SystemData, World,
};

use hibitset::BitSet;
use shrev::EventChannel;

use std::any::TypeId;
use std::cell::{Ref, RefMut};
use std::marker::PhantomData;
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
    _marker: PhantomData<C>,
}

impl<C: 'static> SpecializedResource for EventChannel<C> {
    fn reads(types: &mut Vec<TypeId>) {
        types.push(TypeId::of::<EventChannelNamespace<C>>())
    }

    fn writes(types: &mut Vec<TypeId>) {
        types.push(TypeId::of::<EventChannelNamespace<C>>())
    }
}

/// Trait for types which filter reads and writes markers
/// for a resource.
///
/// Types which implement this trait can be used for
/// fine-grained control over the read and write dependencies
/// of a system.
pub trait AccessorAdapter {
    fn reads(_: &mut Vec<TypeId>) {}
    fn writes(_: &mut Vec<TypeId>) {}
}

/// Adapter which only passes through the read dependencies.
#[derive(Copy, Clone, Debug)]
pub struct ReadAdapter<R: SpecializedResource> {
    _marker: PhantomData<R>,
}

impl<R: SpecializedResource> AccessorAdapter for ReadAdapter<R> {
    fn reads(types: &mut Vec<TypeId>) {
        <R as SpecializedResource>::reads(types);
    }
}

/// Adapter which only passes through the write dependencies.
#[derive(Copy, Clone, Debug)]
pub struct WriteAdapter<R: SpecializedResource> {
    _marker: PhantomData<R>,
}

impl<R: SpecializedResource> AccessorAdapter for WriteAdapter<R> {
    fn writes(types: &mut Vec<TypeId>) {
        <R as SpecializedResource>::writes(types);
    }
}

/// Adapter which doesn't pass anything through.
#[derive(Copy, Clone, Debug)]
pub struct NullAdapter<R: SpecializedResource> {
    _marker: PhantomData<R>,
}

impl<R: SpecializedResource> AccessorAdapter for NullAdapter<R> {}

/// Fetch a resource immutably.
///
/// This will panic if the resource doesn't exist but it
/// will create the resource during setup so that doesn't
/// happen.
pub struct Read<'a, R, A = ReadAdapter<R>>
where
    R: Default + 'static,
    A: AccessorAdapter,
{
    inner: ReadExpect<'a, R, A>,
}

impl<'a, R, A> SystemData<'a> for Read<'a, R, A>
where
    R: Default + 'static,
    A: AccessorAdapter,
{
    fn fetch(world: &'a World) -> Self {
        Self {
            inner: ReadExpect::fetch(world),
        }
    }

    fn setup(world: &mut World) {
        world.register_resource_lazy(R::default);
    }

    fn reads(types: &mut Vec<TypeId>) {
        <ReadExpect<R> as SystemData>::reads(types)
    }
    fn writes(types: &mut Vec<TypeId>) {
        <ReadExpect<R> as SystemData>::reads(types)
    }
}

impl<R, A> Deref for Read<'_, R, A>
where
    R: Default + 'static,
    A: AccessorAdapter,
{
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
pub struct Write<'a, R, A = WriteAdapter<R>>
where
    R: Default + 'static,
    A: AccessorAdapter,
{
    inner: WriteExpect<'a, R, A>,
}

impl<'a, R, A> SystemData<'a> for Write<'a, R, A>
where
    R: Default + 'static,
    A: AccessorAdapter,
{
    fn fetch(world: &'a World) -> Self {
        Self {
            inner: WriteExpect::fetch(world),
        }
    }

    fn setup(world: &mut World) {
        world.register_resource_lazy(R::default);
    }

    fn reads(types: &mut Vec<TypeId>) {
        <WriteExpect<R> as SystemData>::reads(types)
    }
    fn writes(types: &mut Vec<TypeId>) {
        <WriteExpect<R> as SystemData>::writes(types)
    }
}

impl<R, A> Deref for Write<'_, R, A>
where
    R: Default + 'static,
    A: AccessorAdapter,
{
    type Target = R;

    fn deref(&self) -> &R {
        &*self.inner
    }
}

impl<R, A> DerefMut for Write<'_, R, A>
where
    R: Default + 'static,
    A: AccessorAdapter,
{
    fn deref_mut(&mut self) -> &mut R {
        &mut *self.inner
    }
}

/// Fetch a resource immutably.
///
/// This will **not** create the resource during setup so if you
/// wish to avoid panics at runtime you will need to register
/// the resource with the world yourself.
pub struct ReadExpect<'a, R, A = ReadAdapter<R>>
where
    R: 'static,
    A: AccessorAdapter,
{
    res: Ref<'a, R>,
    _marker: PhantomData<A>,
}

impl<'a, R, A> SystemData<'a> for ReadExpect<'a, R, A>
where
    R: 'static,
    A: AccessorAdapter,
{
    fn fetch(world: &'a World) -> Self {
        Self {
            res: world.fetch_resource(),
            _marker: PhantomData,
        }
    }

    fn setup(_: &mut World) {}

    fn reads(types: &mut Vec<TypeId>) {
        <A as AccessorAdapter>::reads(types)
    }
    fn writes(types: &mut Vec<TypeId>) {
        <A as AccessorAdapter>::writes(types)
    }
}

impl<R, A> Deref for ReadExpect<'_, R, A>
where
    R: 'static,
    A: AccessorAdapter,
{
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
pub struct WriteExpect<'a, R, A = WriteAdapter<R>>
where
    R: 'static,
    A: AccessorAdapter,
{
    res: RefMut<'a, R>,
    _marker: PhantomData<A>,
}

impl<'a, R, A> SystemData<'a> for WriteExpect<'a, R, A>
where
    R: 'static,
    A: AccessorAdapter,
{
    fn fetch(world: &'a World) -> Self {
        Self {
            res: world.fetch_resource_mut(),
            _marker: PhantomData,
        }
    }

    fn setup(_: &mut World) {}

    fn reads(types: &mut Vec<TypeId>) {
        <A as AccessorAdapter>::reads(types)
    }
    fn writes(types: &mut Vec<TypeId>) {
        <A as AccessorAdapter>::writes(types)
    }
}

impl<R, A> Deref for WriteExpect<'_, R, A>
where
    R: 'static,
    A: AccessorAdapter,
{
    type Target = R;

    fn deref(&self) -> &R {
        &*self.res
    }
}

impl<R, A> DerefMut for WriteExpect<'_, R, A>
where
    R: 'static,
    A: AccessorAdapter,
{
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
        Self {
            storage: world.fetch_storage(),
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
        Self {
            storage: world.fetch_storage_mut(),
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

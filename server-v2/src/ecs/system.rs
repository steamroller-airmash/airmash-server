use std::any::TypeId;
use std::marker::PhantomData;

use super::World;

pub trait SystemBuilder {
    type System: (for<'a> System<'a>) + 'static;
    type Dependencies: SystemDeps;

    fn build(self) -> Self::System;
}

pub trait SystemData<'a> {
    fn fetch(world: &'a World) -> Self;
    fn setup(world: &mut World);

    fn reads(reads: &mut Vec<TypeId>);
    fn writes(writes: &mut Vec<TypeId>);
}

pub trait SystemDeps {
    // Systems that this one wants to run after
    fn dependencies(deps: &mut Vec<TypeId>);

    fn reads(reads: &mut Vec<TypeId>);
    fn writes(writes: &mut Vec<TypeId>);
}

pub trait System<'a> {
    type SystemData: SystemData<'a>;

    fn run(&mut self, data: Self::SystemData);

    fn setup(&mut self, world: &'a mut World) {
        Self::SystemData::setup(world);
    }

    fn fetch(&self, world: &'a mut World) -> Self::SystemData {
        Self::SystemData::fetch(world)
    }
}

pub trait DynSystem {
    fn run(&mut self, world: &mut World);
}

impl<S> DynSystem for S
where
    S: for<'a> System<'a>,
{
    fn run(&mut self, world: &mut World) {
        let data = S::fetch(self, world);
        S::run(self, data);
    }
}

// Default Impls
impl<'a, T> SystemData<'a> for PhantomData<T> {
    fn fetch(_: &'a World) -> Self {
        PhantomData
    }
    fn setup(_: &mut World) {}

    fn reads(_: &mut Vec<TypeId>) {}
    fn writes(_: &mut Vec<TypeId>) {}
}

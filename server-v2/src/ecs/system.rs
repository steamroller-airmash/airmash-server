use std::any::TypeId;

use super::World;

pub trait SystemBuilder {
    type System: (for<'a> System<'a>) + 'static;
    type Dependencies: SystemDeps;

    fn build(self) -> Self::System;
}

pub trait SystemData<'a> {
    fn fetch(world: &'a mut World) -> Self;
    fn setup(world: &'a mut World) -> Self;

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

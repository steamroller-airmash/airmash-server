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

macro_rules! impl_sys_deps {
    ($first:ident) => {
        impl SystemDeps for () {
            fn dependencies(_: &mut Vec<TypeId>) {}

            fn reads(_: &mut Vec<TypeId>) {}
            fn writes(_: &mut Vec<TypeId>) {}
        }
    };
    ( $first:ident $(, $rest:ident )*) => {
        impl_sys_deps!( $( $rest ),*);
        impl_sys_deps![$first, $($rest,)*];
    };
    [ $( $ty:ident ),* $(,)? ] => {
        impl<$( $ty ),*> SystemDeps for ( $( $ty ),*)
        where
            $( $ty: SystemDeps ),*
        {
            fn dependencies(deps: &mut Vec<TypeId>) {
                $( <$ty as SystemDeps>::dependencies(deps); )*
            }

            fn reads(types: &mut Vec<TypeId>) {
                $( <$ty as SystemDeps>::reads(types); )*
            }
            fn writes(types: &mut Vec<TypeId>) {
                $( <$ty as SystemDeps>::writes(types); )*
            }
        }
    }
}

impl_sys_deps!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

macro_rules! impl_sys_data {
    ($first:ident) => {
        impl<'a> SystemData<'a> for () {
            fn fetch(_: &'a World) -> Self {}
            fn setup(_: &mut World) {}

            fn reads(_: &mut Vec<TypeId>) {}
            fn writes(_: &mut Vec<TypeId>) {}
        }
    };
    ( $first:ident $(, $rest:ident )*) => {
        impl_sys_data!( $( $rest ),*);
        impl_sys_data![$first, $($rest,)*];
    };
    [ $( $ty:ident ),* $(,)? ] => {
        impl<'a $(, $ty )*> SystemData<'a> for ( $( $ty ),*)
        where
            $( $ty: SystemData<'a> ),*
        {
            fn fetch(world: &'a World) -> Self {
                ( $( <$ty as SystemData>::fetch(world) ),* )
            }
            fn setup(world: &mut World) {
                $( <$ty as SystemData>::setup(world); )*
            }

            fn reads(types: &mut Vec<TypeId>) {
                $( <$ty as SystemData>::reads(types); )*
            }
            fn writes(types: &mut Vec<TypeId>) {
                $( <$ty as SystemData>::writes(types); )*
            }
        }
    }
}

impl_sys_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

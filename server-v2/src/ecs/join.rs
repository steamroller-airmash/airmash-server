use crate::ecs::prelude::*;
use crate::ecs::{Component, Entity, EntityStorage, EntityStorageMut, Storage};

use hibitset::{BitIter, BitSet, BitSetLike};

use std::marker::PhantomData;

pub trait Join {
    type Item;
    type Value;
    type Mask: BitSetLike;

    /// Split this joinable object into a mask and
    unsafe fn split(self) -> (Self::Mask, Self::Value);
    unsafe fn get(value: &mut Self::Value, index: u32) -> Self::Item;

    fn join(self) -> JoinIter<Self>
    where
        Self: Sized,
    {
        unsafe { JoinIter::new(self) }
    }
}

pub struct JoinIter<J: Join> {
    mask: BitIter<J::Mask>,
    value: J::Value,
}

impl<J: Join> JoinIter<J> {
    pub unsafe fn new(join: J) -> Self {
        let (mask, value) = join.split();
        Self {
            mask: mask.iter(),
            value,
        }
    }
}

impl<J: Join> Iterator for JoinIter<J> {
    type Item = J::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.mask
            .next()
            .map(|idx| unsafe { J::get(&mut self.value, idx) })
    }
}

pub struct SplitStorage<'a, C: Component> {
    storage: *const C::Storage,
    _marker: PhantomData<&'a C::Storage>,
}

impl<'a, C: Component> SplitStorage<'a, C> {
    unsafe fn get_unchecked(&self, index: u32) -> &'a C {
        (*self.storage).get_unchecked(index)
    }
}

pub struct SplitStorageMut<'a, C: Component> {
    storage: *mut C::Storage,
    _marker: PhantomData<&'a mut C::Storage>,
}

impl<'a, C: Component> SplitStorageMut<'a, C> {
    unsafe fn get_mut_unchecked(&mut self, index: u32) -> &'a mut C {
        (*self.storage).get_mut_unchecked(index)
    }
}

fn split<'a, C: Component>(storage: &'a C::Storage) -> (&'a BitSet, SplitStorage<'a, C>) {
    let mask = storage.mask();
    let value = SplitStorage {
        storage: storage as *const _,
        _marker: PhantomData,
    };

    (mask, value)
}

fn split_mut<'a, C: Component>(
    storage: &'a mut C::Storage,
) -> (&'a BitSet, SplitStorageMut<'a, C>) {
    let value = SplitStorageMut {
        storage: storage as *mut _,
        _marker: PhantomData,
    };
    let mask = storage.mask();

    (mask, value)
}

impl<'a, C> Join for &'a ReadStorage<'_, C>
where
    C: Component,
{
    type Item = &'a C;
    type Value = SplitStorage<'a, C>;
    type Mask = &'a BitSet;

    unsafe fn split(self) -> (Self::Mask, Self::Value) {
        split::<C>(self.storage())
    }

    unsafe fn get(value: &mut Self::Value, index: u32) -> Self::Item {
        value.get_unchecked(index)
    }
}

impl<'a, C> Join for &'a WriteStorage<'_, C>
where
    C: Component,
{
    type Item = &'a C;
    type Value = SplitStorage<'a, C>;
    type Mask = &'a BitSet;

    unsafe fn split(self) -> (Self::Mask, Self::Value) {
        split::<C>(self.storage())
    }

    unsafe fn get(value: &mut Self::Value, index: u32) -> Self::Item {
        value.get_unchecked(index)
    }
}

impl<'a> Join for &'a Entities<'_> {
    type Item = Entity;
    type Value = &'a Entities<'a>;
    type Mask = BitSet;

    unsafe fn split(self) -> (Self::Mask, Self::Value) {
        (self.alive().clone(), self)
    }
    unsafe fn get(value: &mut Self::Value, index: u32) -> Self::Item {
        value.forge(index)
    }
}

impl<'a, C> Join for &'a mut WriteStorage<'_, C>
where
    C: Component,
{
    type Item = &'a mut C;
    type Value = SplitStorageMut<'a, C>;
    type Mask = &'a BitSet;

    unsafe fn split(self) -> (Self::Mask, Self::Value) {
        split_mut::<C>(self.storage_mut())
    }

    unsafe fn get(value: &mut Self::Value, index: u32) -> Self::Item {
        value.get_mut_unchecked(index)
    }
}

macro_rules! bitand {
	( $first:ty ) => { $first };
	( $first:ty $( , $rest:ty )* ) => {
		hibitset::BitSetAnd<$first, bitand!( $($rest),*)>
	};
	[ $first:expr ] => { $first };
	[ $first:expr $(, $rest:expr )*] => {
		hibitset::BitSetAnd($first, bitand![$($rest),*])
	}
}

macro_rules! tuple_impl {
	[__internal: $( $name:ident ),* ] => {
		#[allow(non_snake_case)]
		impl<'a, $($name),* > Join for ( $($name,)* )
		where
			$( $name: Join ),*
		{
			type Item = ( $( <$name as Join>::Item, )* );
			type Value = ( $( <$name as Join>::Value, )* );
			type Mask = bitand!( $( <$name as Join>::Mask ),*);

			unsafe fn split(self) -> (Self::Mask, Self::Value) {
				let ( $($name,)* ) = self;

				$( let $name = $name.split(); )*

				(bitand![$( $name.0 ),*], ( $( $name.1, )* ))
			}

			unsafe fn get(value: &mut Self::Value, index: u32) -> Self::Item {
				let ( $( $name, )* ) = value;

				( $( <$name as Join>::get($name, index), )* )
			}
		}
	};
	( $first:ident ) => {
		tuple_impl![__internal: $first];
	};
	( $first:ident $(, $rest:ident )* $(,)? ) => {
		tuple_impl!($( $rest ),*);
		tuple_impl![__internal: $first $(, $rest )* ];
	}
}

macro_rules! bitset_impl {
	[ $( match for<$( $param:ident, )*> )? $name:ty ] => {
		impl$(< $($param),* >)? Join for $name
		$( where $( $param: BitSetLike, )* )?
		{
			type Item = ();
			type Value = ();
			type Mask = Self;

			unsafe fn split(self) -> (Self::Mask, Self::Value) {
				(self, ())
			}

			unsafe fn get(_: &mut Self::Value, _: u32) {}
		}
	};
	{
		$(
			$( match for<$( $param:ident ),*> )? $name:ty ;
		)*
	} => {
		$( bitset_impl![ $( match for<$( $param, )*> )? $name ]; )*
	}
}

bitset_impl! {
    BitSet;
    hibitset::BitSetAll;
    match for<B> hibitset::BitSetNot<B>;
    match for<A, B> hibitset::BitSetAnd<A, B>;
    match for<A, B> hibitset::BitSetOr<A, B>;
    match for<A, B> hibitset::BitSetXor<A, B>;
}

tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

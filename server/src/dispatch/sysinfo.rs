use specs::{System, World};
use std::any::{Any, TypeId};

use crate::utils::EventDeps;

trait SpecializeDeps {
	fn writes() -> Vec<TypeId>;
	fn reads() -> Vec<TypeId>;
}

struct DepsMarker<T: ?Sized>(std::marker::PhantomData<T>);
struct TupleMarker<T: ?Sized>(std::marker::PhantomData<T>);

impl<T: ?Sized> SpecializeDeps for T {
	default fn writes() -> Vec<TypeId> {
		vec![]
	}
	default fn reads() -> Vec<TypeId> {
		vec![]
	}
}

impl<'a, T> SpecializeDeps for DepsMarker<T>
where
	T: System<'a>,
	<T as System<'a>>::SystemData: EventDeps,
{
	fn writes() -> Vec<TypeId> {
		<<T as System<'a>>::SystemData as EventDeps>::writes()
	}
	fn reads() -> Vec<TypeId> {
		<<T as System<'a>>::SystemData as EventDeps>::reads()
	}
}

impl<T> SpecializeDeps for TupleMarker<T>
where
	T: EventDeps,
{
	fn writes() -> Vec<TypeId> {
		<T as EventDeps>::writes()
	}
	fn reads() -> Vec<TypeId> {
		<T as EventDeps>::reads()
	}
}

pub trait SystemDeps {
	fn dependencies() -> Vec<&'static str>;

	fn reads_events() -> Vec<TypeId> {
		let mut v = <DepsMarker<Self> as SpecializeDeps>::reads();
		v.append(&mut Self::pseudo_reads());
		v
	}
	fn writes_events() -> Vec<TypeId> {
		<DepsMarker<Self> as SpecializeDeps>::writes()
	}

	fn pseudo_reads() -> Vec<TypeId> {
		Vec::new()
	}
}

pub trait SystemInfo {
	type Dependencies: SystemDeps;

	fn name() -> &'static str {
		std::any::type_name::<Self>()
	}
	fn new() -> Self;
	fn new_args(_args: Box<dyn Any>) -> Self
	where
		Self: Sized,
	{
		Self::new()
	}

	fn static_setup(_: &mut World) {}
}

impl<T> SystemDeps for T
where
	T: SystemInfo,
{
	fn dependencies() -> Vec<&'static str> {
		vec![T::name()]
	}
}

macro_rules! decl_tuple {
	{
		$(
			(
				$($param:ident),*
			);
		)*
	} => {
		$(
			impl<$($param,)*> SystemDeps for ($($param,)*)
			where $($param: SystemDeps,)*
			{
				fn dependencies() -> Vec<&'static str> {
					#[allow(unused_mut)]
					let mut deps = vec![];

					$(
						deps.append(&mut $param::dependencies());
					)*

					deps
				}

				fn reads_events() -> Vec<TypeId> {
					<TupleMarker<Self> as SpecializeDeps>::reads()
				}
				fn writes_events() -> Vec<TypeId> {
					<TupleMarker<Self> as SpecializeDeps>::writes()
				}

				fn pseudo_reads() -> Vec<TypeId> {
					#[allow(unused_mut)]
					let mut reads = vec![];

					$(
						reads.append(&mut $param::pseudo_reads());
					)*

					reads
				}
			}

			impl<$($param,)*> EventDeps for ($($param,)*)
			where
				$($param: EventDeps,)*
			{
				fn writes() -> Vec<TypeId> {
					#[allow(unused_mut)]
					let mut writes = vec![];

					$(
						writes.append(&mut <$param as EventDeps>::writes());
					)*

					writes
				}

				fn reads() -> Vec<TypeId> {
					#[allow(unused_mut)]
					let mut reads = vec![];

					$(
						reads.append(&mut <$param as EventDeps>::reads());
					)*

					reads
				}
			}
		)*
	}
}

macro_rules! decl_tuples {
	( [ $( $param:ident ),* ], ) => {
		decl_tuple! {
			( $( $param ),* );
		}
	};
	( [ $( $front:ident ),* ], $current:ident $(, $rest:ident )* ) => {
		decl_tuple! {
			( $( $front ),* );
		}

		decl_tuples!( [ $( $front, )* $current ], $( $rest ),* );
	};
	( $( $param:ident ),* ) => {
		decl_tuples!([], $( $param ),*);
	}
}

// Alphabet pyramid, implement for every tuple up to 26 elements
decl_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

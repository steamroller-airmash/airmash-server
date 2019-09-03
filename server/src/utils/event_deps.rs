use shrev::EventChannel;
use specs::{Component, Entities, Read, ReadExpect, ReadStorage, Write, WriteExpect, WriteStorage};
use std::any::TypeId;

/// Utility trait used in scheduling systems.
/// Indicates which event channels a system writes
/// and reads.
pub trait EventDeps {
	fn writes() -> Vec<TypeId> {
		vec![]
	}
	fn reads() -> Vec<TypeId> {
		vec![]
	}
}

impl<'a, T: Component> EventDeps for ReadStorage<'a, T> {}
impl<'a, T: Component> EventDeps for WriteStorage<'a, T> {}
impl<'a> EventDeps for Entities<'a> {}

impl<'a, T> EventDeps for Read<'a, T> {
	default fn reads() -> Vec<TypeId> {
		vec![]
	}
}
impl<'a, E: 'static> EventDeps for Read<'a, EventChannel<E>> {
	fn reads() -> Vec<TypeId> {
		vec![TypeId::of::<E>()]
	}
}

impl<'a, T> EventDeps for ReadExpect<'a, T> {
	default fn reads() -> Vec<TypeId> {
		vec![]
	}
}
impl<'a, E: 'static> EventDeps for ReadExpect<'a, EventChannel<E>> {
	fn reads() -> Vec<TypeId> {
		vec![TypeId::of::<E>()]
	}
}

impl<'a, T> EventDeps for Write<'a, T> {
	default fn writes() -> Vec<TypeId> {
		vec![]
	}
}
impl<'a, E: 'static> EventDeps for Write<'a, EventChannel<E>> {
	fn writes() -> Vec<TypeId> {
		vec![TypeId::of::<E>()]
	}
}

impl<'a, T> EventDeps for WriteExpect<'a, T> {
	default fn writes() -> Vec<TypeId> {
		vec![]
	}
}
impl<'a, E: 'static> EventDeps for WriteExpect<'a, EventChannel<E>> {
	fn writes() -> Vec<TypeId> {
		vec![TypeId::of::<E>()]
	}
}

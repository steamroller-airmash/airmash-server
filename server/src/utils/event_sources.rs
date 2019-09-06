use std::any::TypeId;
use std::marker::PhantomData;

use crate::SystemDeps;

pub struct EventSources<T: 'static>(PhantomData<T>);

impl<T: 'static> SystemDeps for EventSources<T> {
	fn dependencies() -> Vec<&'static str> {
		Vec::new()
	}

	fn pseudo_reads() -> Vec<TypeId> {
		vec![TypeId::of::<T>()]
	}
}

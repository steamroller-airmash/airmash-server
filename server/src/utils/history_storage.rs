use std::ops::DerefMut;

use specs::{
	error::Error,
	prelude::*,
	storage::{MaskedStorage, Storage},
};

use crate::component::flag::IsZombie;

pub trait HistoricalStorageExt {
	fn insert_with_history(&mut self, id: Entity, value: IsZombie) -> Result<(), Error>;
}

impl<'a, D> HistoricalStorageExt for Storage<'a, IsZombie, D>
where
	D: DerefMut<Target = MaskedStorage<IsZombie>>,
{
	fn insert_with_history(&mut self, e: Entity, value: IsZombie) -> Result<(), Error> {
		if let Some(existing) = self.get_mut(e) {
			existing.merge(value);
			Ok(())
		} else {
			self.insert(e, value).map(|_| ())
		}
	}
}

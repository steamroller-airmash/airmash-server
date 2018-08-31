use types::Team;

/// A flag ID
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Flag(pub Team);

wrapper_serde_decl!(Flag);

#[cfg(features = "specs")]
mod specs_convert {
	use super::Flag;
	use error::EntityIdOutOfRangeError;
	use specs::Entity;
	use std::convert::TryFrom;

	impl TryFrom<Entity> for Flag {
		type Error = EntityIdOutOfRangeError;

		fn try_from(ent: Entity) -> Result<Self, Self::Error> {
			Ok(Flag(TryFrom::try_from(ent.id())?))
		}
	}
}

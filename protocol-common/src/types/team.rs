#[cfg(feature = "specs")]
use specs::DenseVecStorage;

/// Type-safe team identifier
#[derive(Copy, Clone, Eq, Hash, Debug, PartialEq)]
#[cfg_attr(feature = "specs", derive(Component))]
pub struct Team(pub u16);

wrapper_serde_decl!(Team);

#[cfg(feature = "specs")]
mod specs_convert {
	use super::Team;
	use error::EntityIdOutOfRangeError;
	use specs::Entity;
	use std::convert::TryFrom;

	impl TryFrom<Entity> for Team {
		type Error = EntityIdOutOfRangeError;

		fn try_from(ent: Entity) -> Result<Self, Self::Error> {
			Ok(Team(TryFrom::try_from(ent.id())?))
		}
	}
}

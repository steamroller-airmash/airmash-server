#[cfg(feature = "specs")]
use specs::DenseVecStorage;

/// A player ID
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "specs", derive(Component))]
pub struct Player(pub u16);

wrapper_serde_decl!(Player);

#[cfg(feature = "specs")]
mod specs_convert {
	use super::Player;
	use error::EntityIdOutOfRangeError;
	use specs::Entity;
	use std::convert::TryFrom;

	impl TryFrom<Entity> for Player {
		type Error = EntityIdOutOfRangeError;

		fn try_from(ent: Entity) -> Result<Self, Self::Error> {
			Ok(Player(TryFrom::try_from(ent.id())?))
		}
	}
}

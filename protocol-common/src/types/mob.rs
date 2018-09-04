#[cfg(feature = "specs")]
use specs::DenseVecStorage;

/// A mob (missile, upgrade, or powerup) ID
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "specs", derive(Component))]
pub struct Mob(pub u16);

wrapper_serde_decl!(Mob);

#[cfg(feature = "specs")]
mod specs_convert {
	use super::Mob;
	use error::EntityIdOutOfRangeError;
	use specs::Entity;
	use std::convert::TryFrom;

	impl TryFrom<Entity> for Mob {
		type Error = EntityIdOutOfRangeError;

		fn try_from(ent: Entity) -> Result<Self, Self::Error> {
			Ok(Mob(TryFrom::try_from(ent.id())?))
		}
	}
}

/// A mob (missile, upgrade, or powerup) ID
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Mob(u16);

wrapper_serde_decl!(Mob);

#[cfg(features = "specs")]
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

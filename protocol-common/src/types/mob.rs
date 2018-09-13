#[cfg(feature = "specs")]
use specs::DenseVecStorage;

/// A mob (missile, upgrade, or powerup) ID
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "specs", derive(Component))]
pub struct Mob(pub u16);

wrapper_serde_decl!(Mob);

#[cfg(feature = "specs")]
mod specs_convert {
	use super::*;
	use specs::Entity;
	use std::convert::TryInto;

	impl From<Entity> for Mob {
		fn from(ent: Entity) -> Self {
			Mob(ent.id().try_into().expect("Entity id out of range"))
		}
	}
}

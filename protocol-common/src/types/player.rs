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
	use specs::Entity;
	use std::convert::TryInto;

	impl From<Entity> for Player {
		fn from(ent: Entity) -> Self {
			Player(ent.id().try_into().expect("Entity id out of range"))
		}
	}
}

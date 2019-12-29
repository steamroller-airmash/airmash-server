use crate::ecs::{NullStorage, HashMapStorage};

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsPlayer;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsMissile;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsPowerup;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsSpectating;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsDead;

/// Indicates that the missile's lifetime ended,
/// but it is being retained to work around client
/// bugs.
///
/// This component is meant for debugging purposes
#[derive(Clone, Debug, Default, Component)]
#[storage(HashMapStorage)]
pub struct IsZombie {
	/// The system that removed the object
	deleted_by: Vec<&'static str>,
}

impl IsZombie {
	fn new(sys: &'static str) -> Self {
		Self {
			deleted_by: vec![sys],
		}
	}

	pub fn from_sys<S>() -> Self {
		Self::new(std::any::type_name::<S>())
	}

	pub fn merge(&mut self, mut other: IsZombie) {
		self.deleted_by.append(&mut other.deleted_by);
	}
}

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsChatThrottled;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsChatMuted;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct ForcePlayerUpdate;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsBoosting;

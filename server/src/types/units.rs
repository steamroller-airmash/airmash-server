use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

use specs::DenseVecStorage;

pub use dimensioned::{Cbrt, Recip, Root, Sqrt};

pub use protocol_common::{
	Accel, AccelScalar, BaseType, Distance, Energy, EnergyRegen, Flag, Health, HealthRegen, Level,
	Position, Rotation, RotationRate, Score, Speed, Team, Time, Velocity,
};
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default, Component, Ord, PartialOrd)]
pub struct ConnectionId(pub usize);
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default, Component, Ord, PartialOrd)]
pub struct UpgradeCount(pub u16);

static CONNECTION_ID: AtomicUsize = ATOMIC_USIZE_INIT;

impl ConnectionId {
	pub fn new() -> Self {
		ConnectionId(CONNECTION_ID.fetch_add(1, Ordering::Relaxed))
	}
}

use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::time::Duration;

use specs::DenseVecStorage;

use types::Vector2;

pub use dimensioned::{Cbrt, Recip, Root, Sqrt};

pub type BaseType = f32;

pub mod detail {
	use types::BaseType;

	use specs::{Component, VecStorage};

	make_units! {
		AirmashUnits;
		ONE: Unit;

		base {
			D: Distance, "distance";
			S: Time,     "time";
			H: Health,   "health";
			E: Energy,   "energy";
			R: Rotation, "rotation";
		}

		derived {
			HR: HealthRegen = (Health / Time);
			ER: EnergyRegen = (Energy / Time);
			V:  Speed       = (Distance / Time);
			A:  Accel       = (Speed / Time);
			RR: RotationRate = (Rotation / Time);
		}

		constants {

		}

		fmt = true;
	}

	impl<T: Clone, U> AirmashUnits<T, U> {
		pub fn inner(&self) -> T {
			self.value_unsafe.clone()
		}
	}

	impl<U> AirmashUnits<BaseType, U> {
		pub fn abs(self) -> Self {
			Self::new(self.inner().abs())
		}
		pub fn signum(self) -> BaseType {
			self.inner().signum()
		}

		pub fn sin_cos(self) -> (BaseType, BaseType) {
			self.inner().sin_cos()
		}
		pub fn max(self, o: Self) -> Self {
			Self::new(self.inner().max(o.inner()))
		}
		pub fn min(self, o: Self) -> Self {
			Self::new(self.inner().min(o.inner()))
		}
	}

	impl<T: 'static, U: 'static> Component for AirmashUnits<T, U>
	where
		T: Sync + Send,
		U: Sync + Send,
	{
		type Storage = VecStorage<AirmashUnits<T, U>>;
	}

	impl<T: Default, U> Default for AirmashUnits<T, U> {
		fn default() -> Self {
			Self::new(T::default())
		}
	}
}

pub type Distance = detail::Distance<BaseType>;
pub type Time = detail::Time<BaseType>;
pub type Health = detail::Health<BaseType>;
pub type Energy = detail::Energy<BaseType>;
pub type Rotation = detail::Rotation<BaseType>;
pub type Position = Vector2<Distance>;

pub type HealthRegen = detail::HealthRegen<BaseType>;
pub type EnergyRegen = detail::EnergyRegen<BaseType>;
pub type Velocity = Vector2<detail::Speed<BaseType>>;
pub type Accel = Vector2<detail::Accel<BaseType>>;
pub type RotationRate = detail::RotationRate<BaseType>;
pub type Speed = detail::Speed<BaseType>;
pub type AccelScalar = detail::Accel<BaseType>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default, Component, Ord, PartialOrd)]
pub struct Team(pub u16);
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default, Component, Ord, PartialOrd)]
pub struct Level(pub u8);
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default, Component, Ord, PartialOrd)]
pub struct Score(pub u32);
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default, Component, Ord, PartialOrd)]
pub struct ConnectionId(pub usize);
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default, Component, Ord, PartialOrd)]
pub struct UpgradeCount(pub u16);

impl From<Duration> for Time {
	fn from(dt: Duration) -> Time {
		Time::new(dt.as_secs() as BaseType + 1.0e-9 * (dt.subsec_nanos() as BaseType)) * 60.0
	}
}

impl Rotation {
	pub fn sin(&self) -> BaseType {
		self.inner().sin()
	}
	pub fn cos(&self) -> BaseType {
		self.inner().cos()
	}
	pub fn tan(&self) -> BaseType {
		self.inner().tan()
	}
}

static CONNECTION_ID: AtomicUsize = ATOMIC_USIZE_INIT;

impl ConnectionId {
	pub fn new() -> Self {
		ConnectionId(CONNECTION_ID.fetch_add(1, Ordering::Relaxed))
	}
}

// Implement new for all custom types (for consistency)
impl Team {
	fn new(t: u16) -> Self {
		Team(t)
	}
}
impl Level {
	fn new(t: u8) -> Self {
		Level(t)
	}
}
impl Score {
	fn new(t: u32) -> Self {
		Score(t)
	}
}

impl Position {
	pub fn rotate(self, angle: Rotation) -> Self {
		let (sin, cos) = angle.sin_cos();

		Position::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
	}
}

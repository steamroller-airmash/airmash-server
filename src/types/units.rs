
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

use dimensioned::{Cbrt, Sqrt, Root, Recip};

use types::Vector2;

pub type BaseType = f64;

pub mod detail {
	use types::units::BaseType;
	
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
		pub fn abs(&self) -> Self {
			Self::new(self.inner().abs())
		}
		pub fn signum(&self) -> BaseType {
			self.inner().signum()
		}
	}
}

pub type Distance = detail::Distance<BaseType>;
pub type Time     = detail::Time<BaseType>;
pub type Health   = detail::Health<BaseType>;
pub type Energy   = detail::Energy<BaseType>;
pub type Rotation = detail::Rotation<BaseType>;
pub type Position = Vector2<Distance>;

pub type HealthRegen  = detail::HealthRegen<BaseType>;
pub type EnergyRegen  = detail::EnergyRegen<BaseType>;
pub type Speed        = Vector2<detail::Speed<BaseType>>;
pub type Accel        = Vector2<detail::Accel<BaseType>>;
pub type RotationRate = detail::RotationRate<BaseType>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct Team(pub u16);
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct Level(pub u16);
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct Score(pub u32);
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct ConnectionId(pub usize);

impl From<Duration> for Time {
	fn from(dt: Duration) -> Time {
		Time::new(dt.as_secs() as BaseType 
			+ 1.0e-9 * (dt.subsec_nanos() as BaseType))
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
		ConnectionId(
			CONNECTION_ID.fetch_add(1, Ordering::Relaxed)
		)
	}
}

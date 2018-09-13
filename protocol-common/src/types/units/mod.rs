mod base;

pub use self::base::{AirmashUnits, BaseType};
use types::Vector2;

pub type Distance = self::base::Distance<BaseType>;
pub type Time = self::base::Time<BaseType>;
pub type Health = self::base::Health<BaseType>;
pub type Energy = self::base::Energy<BaseType>;
pub type Rotation = self::base::Rotation<BaseType>;

pub type HealthRegen = self::base::HealthRegen<BaseType>;
pub type EnergyRegen = self::base::EnergyRegen<BaseType>;
pub type Speed = self::base::Speed<BaseType>;
pub type AccelScalar = self::base::Accel<BaseType>;
pub type RotationRate = self::base::RotationRate<BaseType>;

pub type Position = Vector2<Distance>;
pub type Velocity = Vector2<Speed>;
pub type Accel = Vector2<AccelScalar>;

impl Position {
	pub fn rotate(self, angle: Rotation) -> Self {
		let (sin, cos) = angle.sin_cos();

		Position::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
	}
}

mod base;

pub use self::base::{AirmashUnits, BaseType};
use crate::types::Vector2;

/// On-map distances.
///
/// While this vector can represent any distance or
/// position that fits within the float, the range
/// of coordinates on the map is limited to the
/// ranges `[-16384, 16384]` for `x` coordiantes and
/// `[-8192, 8192]` for `y` coordinates.
pub type Distance = self::base::Distance<BaseType>;
/// Time unit. (1 unit of time ~= 16.667ms)
///
/// Usually you will want [`Duration`][0] instead
/// of this. This unit is only relevant when doing
/// physics calculations. An implementation of
/// [`From`][1] is implemented to convert from
/// [`Duration`s][0] when needed.
///
/// [0]: https://doc.rust-lang.org/std/time/struct.Duration.html
/// [1]: https://doc.rust-lang.org/std/convert/trait.From.html
pub type Time = self::base::Time<BaseType>;
/// Health unit.
///
/// This is used to represent the health of all players.
/// While this unit can represent any floating point
/// value, the only values that are valid when sent to
/// the client are in the range `[0, 1]`.
pub type Health = self::base::Health<BaseType>;
/// Energy unit.
///
/// Used to represent the energy of all players.
/// While this unit can represent any floating point
/// value, only values in the range `[0, 1]` are valid
/// when sent to the client.
pub type Energy = self::base::Energy<BaseType>;
/// Unit for rotations (in radians).
///
/// While this unit can represent any floating point
/// value, only values in the range `[0, ~10)` will
/// be sent properly to the client.
pub type Rotation = self::base::Rotation<BaseType>;

/// Unit of `Health / Time`.
///
/// Represents how fast a plane regenerates lost health.
pub type HealthRegen = self::base::HealthRegen<BaseType>;
/// Unit of `Energy / Time`.
///
/// Represents how fast a plane regenerates used energy.
pub type EnergyRegen = self::base::EnergyRegen<BaseType>;
/// Unit of velocity: `Distance / Time`.
///
/// Represents how fast a plane is moving.
pub type Speed = self::base::Speed<BaseType>;
/// Unit of acceleration: `Distance / Time^2`.
pub type AccelScalar = self::base::Accel<BaseType>;
/// Unit of angular velocity: `Rotation / Time`.
pub type RotationRate = self::base::RotationRate<BaseType>;

/// A 2D vector of [`Distance`]s.
pub type Position = Vector2<Distance>;
/// A 2D vector of [`Speed`]s.
pub type Velocity = Vector2<Speed>;
/// A 2D vector of [`AccelScalar`]s.
pub type Accel = Vector2<AccelScalar>;

impl Position {
	/// Rotate a vector around the origin by `angle`.
	pub fn rotate(self, angle: Rotation) -> Self {
		let (sin, cos) = angle.sin_cos();

		Position::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
	}
}

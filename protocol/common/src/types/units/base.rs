#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "specs")]
use specs::{Component, VecStorage};

use std::time::Duration;

/// Inner type used for all unit type declarations.
///
/// All units can be converted into this type by
/// calling the [`inner()`][0] method.
///
/// [0]: struct.AirmashUnits.html#method.inner
pub type BaseType = f32;

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
	/// Access the inner type of the unit.
	pub fn inner(&self) -> T {
		self.value_unsafe.clone()
	}
}

impl<U> AirmashUnits<BaseType, U> {
	/// Absolute value
	pub fn abs(self) -> Self {
		Self::new(self.inner().abs())
	}
	/// Get the sign of the inner value of the unit.
	pub fn signum(self) -> BaseType {
		self.inner().signum()
	}

	/// Calculate the max of two values with the same
	/// units.
	pub fn max(self, o: Self) -> Self {
		Self::new(self.inner().max(o.inner()))
	}
	/// Calculate the min of two values with the same
	/// units.
	pub fn min(self, o: Self) -> Self {
		Self::new(self.inner().min(o.inner()))
	}

	/// Combined sin and cos, can be done more
	/// efficiently then doing both calculations
	/// on their own.
	pub fn sin_cos(self) -> (BaseType, BaseType) {
		self.inner().sin_cos()
	}
	/// Calculate the sine of the inner value.
	pub fn sin(&self) -> BaseType {
		self.inner().sin()
	}
	/// Calculate the cosine of the inner value.
	pub fn cos(&self) -> BaseType {
		self.inner().cos()
	}
	/// Calculate the tangent of the inner value.
	pub fn tan(&self) -> BaseType {
		self.inner().tan()
	}
}

#[cfg(feature = "specs")]
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

impl<T, U> From<T> for AirmashUnits<T, U> {
	fn from(v: T) -> Self {
		Self::new(v)
	}
}

#[cfg(feature = "serde")]
impl<T, U> Serialize for AirmashUnits<T, U>
where
	T: Serialize,
{
	fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		self.value_unsafe.serialize(ser)
	}
}

#[cfg(feature = "serde")]
impl<'de, T, U> Deserialize<'de> for AirmashUnits<T, U>
where
	T: Deserialize<'de>,
{
	fn deserialize<D>(de: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		Ok(T::deserialize(de)?.into())
	}
}

impl From<Duration> for Time<BaseType> {
	fn from(dt: Duration) -> Time<BaseType> {
		Time::new(dt.as_secs() as BaseType + 1.0e-9 * (dt.subsec_nanos() as BaseType)) * 60.0
	}
}
impl Into<Duration> for Time<BaseType> {
	fn into(self) -> Duration {
		Duration::from_nanos((self.inner() * (1.0e9 / 60.0)) as u64)
	}
}

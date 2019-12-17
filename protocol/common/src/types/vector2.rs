use std::ops::*;

use dimensioned::Sqrt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "specs")]
use specs;

/// Required trait to allow specialized impls for self
/// TODO: Use specialization instead?
#[doc(hidden)]
// The current version of rustfmt will format this into
// a syntax error
#[cfg_attr(rustfmt, rustfmt_skip)]
pub auto trait NotVec {}
impl<T> !NotVec for Vector2<T> {}

/// A 2D Vector that works with unit conversions.
///
/// **Note:** [`Position`][0], [`Velocity`][1],
/// and [`Accel`][2] are all instances of this struct
/// with different units.
///
/// [0]: type.Position.html
/// [1]: type.Velocity.html
/// [2]: type.Accel.html
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Vector2<T> {
	pub x: T,
	pub y: T,
}

impl<T> Vector2<T> {
	/// Create a new vector with components that convert
	/// into the vectors types.
	///
	/// This is easier to use when creating a vector
	/// from components, but it may prevent type inference
	/// when the vector type is not specified.
	///
	/// # Example
	/// ```
	/// # extern crate airmash_protocol;
	/// # use airmash_protocol::*;
	/// # fn main() {
	/// // Note that this is the same as Position
	/// let vec: Vector2<Distance> = Vector2::new(0.0, 0.0);
	/// # }
	/// ```
	pub fn new<X>(x: X, y: X) -> Self
	where
		X: Into<T>,
	{
		Self {
			x: x.into(),
			y: y.into(),
		}
	}

	/// Take the dot product of two vectors.
	///
	/// The dot product for a 2D vector is defined
	/// (given two vectors `a` and `b`) as:
	/// `a.x * b.x + a.y * b.x`.
	///
	/// # Example
	/// ```
	/// # extern crate airmash_protocol;
	/// # use airmash_protocol::*;
	/// # fn main() {
	/// let a: Vector2<i32> = Vector2::new(1, 2);
	/// let b: Vector2<i32> = Vector2::new(3, 4);
	///
	/// let c = Vector2::dot(a, b);
	///
	/// assert_eq!(c, 11);
	/// # }
	/// ```
	pub fn dot<U>(self, rhs: Vector2<U>) -> <<T as Mul<U>>::Output as Add>::Output
	where
		T: Mul<U>,
		<T as Mul<U>>::Output: Add,
	{
		self.x * rhs.x + self.y * rhs.y
	}

	/// Calculate the magnitude of the vector.
	///
	/// # Examples
	/// Calculate the distance between two points.
	/// ```
	/// # extern crate airmash_protocol;
	/// # use airmash_protocol::*;
	/// # fn main() {
	/// let a: Vector2<f32> = Vector2::new(4.0, 0.0);
	/// let b: Vector2<f32> = Vector2::new(-4.0, 0.0);
	///
	/// // The distance is length of the vector going
	/// // from b to a
	/// let dist = (a - b).length();
	///
	/// # // This case should be ok for exact comparisons
	/// assert_eq!(dist, 8.0);
	/// # }
	pub fn length(self) -> <<<T as Mul>::Output as Add>::Output as Sqrt>::Output
	where
		Self: Clone,
		T: Mul,
		T::Output: Add,
		<T::Output as Add>::Output: Sqrt,
	{
		self.length2().sqrt()
	}

	/// Calculate the length squared.
	pub fn length2(self) -> <<T as Mul>::Output as Add>::Output
	where
		Self: Clone,
		T: Mul,
		T::Output: Add,
	{
		Self::dot(self.clone(), self)
	}

	/// Return a vector pointing in the same direction as this one
	/// but with a magniture of 1.
	///
	/// ## Note
	/// When used with units this will always return a dimensionless vector.
	pub fn normalized(
		self,
	) -> Vector2<<T as Div<<<<T as Mul>::Output as Add>::Output as Sqrt>::Output>>::Output>
	where
		Self: Clone + PartialEq + Default,
		T: Mul + Div<<<<T as Mul>::Output as Add>::Output as Sqrt>::Output>,
		<T as Mul>::Output: Add,
		<<T as Mul>::Output as Add>::Output: Sqrt,
		<<<T as Mul>::Output as Add>::Output as Sqrt>::Output: Clone + NotVec,
		<T as Div<<<<T as Mul>::Output as Add>::Output as Sqrt>::Output>>::Output: Default,
	{
		// Avoid returning NaN when using floating point operands
		// and self == (0.0, 0.0)
		if self == Self::default() {
			Default::default()
		} else {
			self.clone() / self.length()
		}
	}
}

impl<T, U> Add<U> for Vector2<T>
where
	T: Add<U>,
	U: Clone + NotVec,
{
	type Output = Vector2<T::Output>;

	fn add(self, rhs: U) -> Self::Output {
		Self::Output::new(self.x + rhs.clone(), self.y + rhs)
	}
}
impl<T, U> Sub<U> for Vector2<T>
where
	T: Sub<U>,
	U: Clone + NotVec,
{
	type Output = Vector2<T::Output>;

	fn sub(self, rhs: U) -> Self::Output {
		Self::Output::new(self.x - rhs.clone(), self.y - rhs)
	}
}
impl<T, U> Mul<U> for Vector2<T>
where
	T: Mul<U>,
	U: Clone + NotVec,
{
	type Output = Vector2<T::Output>;

	fn mul(self, rhs: U) -> Self::Output {
		Self::Output::new(self.x * rhs.clone(), self.y * rhs)
	}
}
impl<T, U> Div<U> for Vector2<T>
where
	T: Div<U>,
	U: Clone + NotVec,
{
	type Output = Vector2<T::Output>;

	fn div(self, rhs: U) -> Self::Output {
		Self::Output::new(self.x / rhs.clone(), self.y / rhs)
	}
}

impl<T, U> Add<Vector2<U>> for Vector2<T>
where
	T: Add<U>,
{
	type Output = Vector2<T::Output>;

	fn add(self, rhs: Vector2<U>) -> Self::Output {
		Self::Output::new(self.x + rhs.x, self.y + rhs.y)
	}
}
impl<T, U> Sub<Vector2<U>> for Vector2<T>
where
	T: Sub<U>,
{
	type Output = Vector2<T::Output>;

	fn sub(self, rhs: Vector2<U>) -> Self::Output {
		Self::Output::new(self.x - rhs.x, self.y - rhs.y)
	}
}
impl<T, U> Mul<Vector2<U>> for Vector2<T>
where
	T: Mul<U>,
{
	type Output = Vector2<T::Output>;

	fn mul(self, rhs: Vector2<U>) -> Self::Output {
		Self::Output::new(self.x * rhs.x, self.y * rhs.y)
	}
}

impl<T, U> AddAssign<U> for Vector2<T>
where
	Self: Add<U, Output = Vector2<T>> + Clone,
{
	fn add_assign(&mut self, rhs: U) {
		*self = self.clone() + rhs;
	}
}
impl<T, U> SubAssign<U> for Vector2<T>
where
	Self: Sub<U, Output = Vector2<T>> + Clone,
{
	fn sub_assign(&mut self, rhs: U) {
		*self = self.clone() - rhs;
	}
}
impl<T, U> MulAssign<U> for Vector2<T>
where
	Self: Mul<U, Output = Vector2<T>> + Clone,
{
	fn mul_assign(&mut self, rhs: U) {
		*self = self.clone() * rhs;
	}
}
impl<T, U> DivAssign<U> for Vector2<T>
where
	Self: Div<U, Output = Vector2<T>> + Clone,
{
	fn div_assign(&mut self, rhs: U) {
		*self = self.clone() / rhs;
	}
}

#[cfg(feature = "specs")]
impl<T: 'static + Send + Sync> specs::Component for Vector2<T> {
	type Storage = specs::VecStorage<Vector2<T>>;
}

#[cfg(feature = "serde")]
impl<T> Serialize for Vector2<T>
where
	T: Serialize + Clone,
{
	fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		(self.x.clone(), self.y.clone()).serialize(s)
	}
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for Vector2<T>
where
	T: Deserialize<'de>,
{
	fn deserialize<D>(de: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let (x, y) = <(T, T)>::deserialize(de)?;
		Ok(Self { x, y })
	}
}

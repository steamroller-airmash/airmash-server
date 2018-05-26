
use std::ops::*;
use std::mem;

use dimensioned::Sqrt;

/// Required trait to allow specialized impls for self
#[doc(hidden)]
pub auto trait NotVec {}
impl<T> !NotVec for Vector2<T> {}


#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Vector2<T> {
	pub x: T,
	pub y: T
}

impl<T> Vector2<T> {
	pub fn new(x: T, y: T) -> Self {
		Self{ x, y }
	}

	pub fn dot<U>(self, rhs: Vector2<U>) -> <<T as Mul<U>>::Output as Add>::Output
	where 
		T: Mul<U>,
		<T as Mul<U>>::Output: Add
	{
		self.x * rhs.x + self.y * rhs.y
	}

	pub fn length(self) -> <<<T as Mul>::Output as Add>::Output as Sqrt>::Output 
	where 
		Self: Clone,
		T: Mul,
		T::Output: Add,
		<T::Output as Add>::Output: Sqrt
	{
		Self::dot(self.clone(), self).sqrt()
	}
}


impl<T, U> Add<U> for Vector2<T> 
where T: Add<U>, U: Clone + NotVec
{
	type Output = Vector2<T::Output>;

	fn add(self, rhs: U) -> Self::Output {
		Self::Output::new(
			self.x + rhs.clone(),
			self.y + rhs
		)
	}
}
impl<T, U> Sub<U> for Vector2<T> 
where T: Sub<U>, U: Clone + NotVec
{
	type Output = Vector2<T::Output>;

	fn sub(self, rhs: U) -> Self::Output {
		Self::Output::new(
			self.x - rhs.clone(),
			self.y - rhs
		)
	}
}
impl<T, U> Mul<U> for Vector2<T> 
where T: Mul<U>, U: Clone + NotVec
{
	type Output = Vector2<T::Output>;

	fn mul(self, rhs: U) -> Self::Output {
		Self::Output::new(
			self.x * rhs.clone(),
			self.y * rhs
		)
	}
}
impl<T, U> Div<U> for Vector2<T> 
where T: Div<U>, U: Clone + NotVec
{
	type Output = Vector2<T::Output>;

	fn div(self, rhs: U) -> Self::Output {
		Self::Output::new(
			self.x / rhs.clone(),
			self.y / rhs
		)
	}
}


impl<T, U> Add<Vector2<U>> for Vector2<T>
where T: Add<U>
{
	type Output = Vector2<T::Output>;

	fn add(self, rhs: Vector2<U>) -> Self::Output {
		Self::Output::new(
			self.x + rhs.x,
			self.y + rhs.y
		)
	}
}
impl<T, U> Sub<Vector2<U>> for Vector2<T>
where T: Sub<U>
{
	type Output = Vector2<T::Output>;

	fn sub(self, rhs: Vector2<U>) -> Self::Output {
		Self::Output::new(
			self.x - rhs.x,
			self.y - rhs.y
		)
	}
}
impl<T, U> Mul<Vector2<U>> for Vector2<T>
where T: Mul<U>
{
	type Output = Vector2<T::Output>;

	fn mul(self, rhs: Vector2<U>) -> Self::Output {
		Self::Output::new(
			self.x * rhs.x,
		 	self.y * rhs.y
		)
	}
}

impl<T, U> AddAssign<U> for Vector2<T> 
where Self: Add<U, Output=Vector2<T>>
{
	fn add_assign(&mut self, rhs: U) {
		let val = mem::replace(self, unsafe { mem::uninitialized() });
		mem::forget(mem::replace(self, val + rhs));
	}
}
impl<T, U> SubAssign<U> for Vector2<T> 
where Self: Sub<U, Output=Vector2<T>>
{
	fn sub_assign(&mut self, rhs: U) {
		let val = mem::replace(self, unsafe { mem::uninitialized() });
		mem::forget(mem::replace(self, val - rhs));
	}
}
impl<T, U> MulAssign<U> for Vector2<T> 
where Self: Mul<U, Output=Vector2<T>>
{
	fn mul_assign(&mut self, rhs: U) {
		let val = mem::replace(self, unsafe { mem::uninitialized() });
		mem::forget(mem::replace(self, val * rhs));
	}
}
impl<T, U> DivAssign<U> for Vector2<T> 
where Self: Div<U, Output=Vector2<T>>
{
	fn div_assign(&mut self, rhs: U) {
		let val = mem::replace(self, unsafe { mem::uninitialized() });
		mem::forget(mem::replace(self, val / rhs));
	}
}


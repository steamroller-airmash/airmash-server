#![allow(dead_code)]

use std::convert::*;
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct MaybeInit<T>(Option<T>);

impl<T> MaybeInit<T> {
	pub fn new<U>(val: U) -> Self
	where
		U: Into<Option<T>>,
	{
		MaybeInit(val.into())
	}

	pub fn uninit() -> Self {
		Self::new(None)
	}

	pub fn init(val: T) -> Self {
		Self::new(val)
	}

	pub fn into_inner(me: Self) -> Option<T> {
		me.0
	}

	pub fn as_ref(me: &Self) -> MaybeInit<&T> {
		MaybeInit::new(me.0.as_ref())
	}

	pub fn as_mut(me: &mut Self) -> MaybeInit<&mut T> {
		MaybeInit::new(me.0.as_mut())
	}

	pub fn is_init(me: &Self) -> bool {
		me.0.is_some()
	}

	pub fn is_uninit(me: &Self) -> bool {
		!Self::is_init(me)
	}
}

impl<T> From<Option<T>> for MaybeInit<T> {
	fn from(val: Option<T>) -> MaybeInit<T> {
		MaybeInit(val)
	}
}

impl<T> Deref for MaybeInit<T> {
	type Target = T;

	fn deref(&self) -> &T {
		match Self::as_ref(self).0 {
			Some(r) => r,
			None => panic!("Attempted to deref an uninintialized MaybeInit"),
		}
	}
}

impl<T> DerefMut for MaybeInit<T> {
	fn deref_mut(&mut self) -> &mut T {
		match Self::as_mut(self).0 {
			Some(r) => r,
			None => panic!("Attempted to deref an uninitialized MaybeInit"),
		}
	}
}

impl<T> AsRef<T> for MaybeInit<T> {
	fn as_ref(&self) -> &T {
		self.deref()
	}
}

impl<T> AsMut<T> for MaybeInit<T> {
	fn as_mut(&mut self) -> &mut T {
		self.deref_mut()
	}
}

impl<T> Default for MaybeInit<T> {
	fn default() -> Self {
		Self::uninit()
	}
}

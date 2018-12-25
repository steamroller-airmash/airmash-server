#![allow(dead_code)]

use std::convert::*;
use std::ops::{Deref, DerefMut};

/// A convenience wrapper for values that need
/// to be initialized at a later time.
///
/// This is a wrapper that will panic if it hasn't
/// been initialized yet and it is dereferenced.
/// The primary use for this is with
/// [`ReaderId`](shrev::ReaderId)s
/// for [`EventChannels`](shrev::EventChannel)
/// which must wait for the
/// system `setup` method to be called before they
/// can be initialized. It eliminates the need to
/// do a long chain of `x.as_mut().unwrap()` when
/// trying to pass a value, instead you can do
/// `&mut *x` and everything works similarly.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct MaybeInit<T>(Option<T>);

impl<T> MaybeInit<T> {
	/// Create a new `MaybeInit` that can either
	/// be initialized or uninitialized.
	pub fn new<U>(val: U) -> Self
	where
		U: Into<Option<T>>,
	{
		MaybeInit(val.into())
	}

	/// Create an uninitialized `MaybeInit`.
	pub fn uninit() -> Self {
		Self::new(None)
	}

	/// Create an initialized `MaybeInit` with a value.
	pub fn init(val: T) -> Self {
		Self::new(val)
	}

	/// Get the inner option representing this `MaybeInit` value.
	pub fn into_inner(me: Self) -> Option<T> {
		me.0
	}

	/// Get a reference to the value, if it is initialized.
	pub fn as_ref(me: &Self) -> MaybeInit<&T> {
		MaybeInit::new(me.0.as_ref())
	}

	/// Get a mutable reference to the value, if it is initialized.
	pub fn as_mut(me: &mut Self) -> MaybeInit<&mut T> {
		MaybeInit::new(me.0.as_mut())
	}

	/// Indicates whether this `MaybeInit` is initialized.
	pub fn is_init(me: &Self) -> bool {
		me.0.is_some()
	}

	/// Inidicates whether this `MaybeInit` is uninitialized.
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

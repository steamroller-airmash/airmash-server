//! All error types for this crate.

use std::num::TryFromIntError;

pub struct EntityIdOutOfRangeError;

impl From<TryFromIntError> for EntityIdOutOfRangeError {
	fn from(_: TryFromIntError) -> Self {
		Self {}
	}
}
impl From<!> for EntityIdOutOfRangeError {
	fn from(never: !) -> Self {
		never
	}
}

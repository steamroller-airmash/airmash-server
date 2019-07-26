use crate::types::*;

use std::marker::PhantomData;

/// Size of the map as a vector.
pub const MAP_SIZE: Vector2<Distance> = Vector2 {
	x: Distance {
		value_unsafe: 32768.0,
		_marker: PhantomData,
	},
	y: Distance {
		value_unsafe: 16384.0,
		_marker: PhantomData,
	},
};

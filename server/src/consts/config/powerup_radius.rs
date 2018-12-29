use std::marker::PhantomData;
use types::Distance;

pub const POWERUP_RADIUS: Distance = Distance {
	value_unsafe: 30.0,
	_marker: PhantomData,
};

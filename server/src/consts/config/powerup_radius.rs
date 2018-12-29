use std::marker::PhantomData;
use types::Distance;

/// The radius of a powerup (for collision purposes).
/// 
/// This is essentially the distance a player must be
/// from a powerup before they are able to pick it up.
pub const POWERUP_RADIUS: Distance = Distance {
	value_unsafe: 30.0,
	_marker: PhantomData,
};

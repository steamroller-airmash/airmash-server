#[cfg(feature = "specs")]
use specs::{Component, DenseVecStorage};

/// All possible "keys" that a player can have activated.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "specs", derive(Component))]
pub struct ServerKeyState {
	pub up: bool,
	pub down: bool,
	pub left: bool,
	pub right: bool,
	pub boost: bool,
	pub strafe: bool,
	pub stealth: bool,
	pub flagspeed: bool,
}

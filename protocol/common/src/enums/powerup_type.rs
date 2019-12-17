#[cfg(feature = "specs")]
use specs::{Component, DenseVecStorage};

/// TODO: Reverse engineer
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Conversions)]
#[cfg_attr(feature = "specs", derive(Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PowerupType {
	Shield = 1,
	/// This is just a guess.
	/// TODO: Verify
	Inferno = 2,
}

impl Default for PowerupType {
	fn default() -> Self {
		PowerupType::Shield
	}
}

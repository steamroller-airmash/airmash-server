#[cfg(feature = "specs")]
use specs::{Component, DenseVecStorage};

/// Specific identifiers for server custom messages.
///
/// TODO: Reverse Engineer
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Conversions)]
#[cfg_attr(feature = "specs", derive(Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ServerCustomType {
	/// TODO: Determine if this name is accurate
	BTRWin = 1,
	/// TODO: Determine if this name is accurate
	CTFWin = 2,
}

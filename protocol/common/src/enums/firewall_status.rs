#[cfg(feature = "specs")]
use specs::{Component, DenseVecStorage};

/// TODO: Reverse engineer
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Primitive)]
#[cfg_attr(feature = "specs", derive(Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FirewallStatus {
	#[doc(hidden)]
	/// Not a real value, just makes derives work
	/// remove this once the enum is reverse engineered
	_Unknown = 0,
}

impl_try_from2!(FirewallStatus);

#[cfg(feature = "specs")]
use specs::DenseVecStorage;

/// Type-safe score identifier
///
/// TODO: Implement arithmetic operations
#[derive(Copy, Clone, Eq, Hash, Debug, Default, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "specs", derive(Component))]
pub struct Score(pub u32);

wrapper_serde_decl!(Score);

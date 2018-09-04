#[cfg(feature = "specs")]
use specs::DenseVecStorage;

/// Type-safe Level identifier
#[derive(Copy, Clone, Eq, Hash, Debug, PartialEq)]
#[cfg_attr(feature = "specs", derive(Component))]
pub struct Level(pub u8);

wrapper_serde_decl!(Level);

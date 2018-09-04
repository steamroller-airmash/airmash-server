#[cfg(feature = "specs")]
use specs::DenseVecStorage;

/// Type-safe team identifier
#[derive(Copy, Clone, Eq, Hash, Debug, PartialEq)]
#[cfg_attr(feature = "specs", derive(Component))]
pub struct Team(pub u16);

wrapper_serde_decl!(Team);

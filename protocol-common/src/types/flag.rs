use types::Team;

/// A flag ID
#[cfg(feature = "specs")]
use specs::DenseVecStorage;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "specs", derive(Component))]
pub struct Flag(pub Team);

wrapper_serde_decl!(Flag);

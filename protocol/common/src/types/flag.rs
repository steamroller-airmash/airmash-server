use crate::types::Team;

#[cfg(feature = "specs")]
use specs::{Component, DenseVecStorage};

/// A flag ID
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, From, Into, Constructor)]
#[cfg_attr(feature = "specs", derive(Component))]
pub struct Flag(pub Team);

wrapper_serde_decl!(Flag);

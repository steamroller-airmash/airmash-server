use crate::types::Team;

/// A flag ID
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, From, Into, Constructor)]
pub struct Flag(pub Team);

wrapper_serde_decl!(Flag);

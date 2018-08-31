/// Type-safe score identifier
///
/// TODO: Implement arithmetic operations
#[derive(Copy, Clone, Eq, Hash, Debug, PartialEq, Ord, PartialOrd)]
pub struct Score(pub u32);

wrapper_serde_decl!(Score);

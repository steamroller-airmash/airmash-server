/// Type-safe score identifier
///
/// TODO: Implement arithmetic operations
#[derive(
    Copy,
    Clone,
    Eq,
    Hash,
    Debug,
    Default,
    PartialEq,
    Ord,
    PartialOrd,
    From,
    Into,
    Add,
    Sub,
    Constructor,
    AddAssign,
)]
pub struct Score(pub u32);

wrapper_serde_decl!(Score);

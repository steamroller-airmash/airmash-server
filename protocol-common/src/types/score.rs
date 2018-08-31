/// Type-safe team identifier
#[derive(Copy, Clone, Eq, Hash, Debug, PartialEq)]
pub struct Score(pub u32);

wrapper_serde_decl!(Score);

/// Type-safe Level identifier
#[derive(Copy, Clone, Eq, Hash, Debug, PartialEq)]
pub struct Level(pub u8);

wrapper_serde_decl!(Level);

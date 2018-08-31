/// Type-safe team identifier
#[derive(Copy, Clone, Eq, Hash, Debug, PartialEq)]
pub struct Team(pub u16);

wrapper_serde_decl!(Team);

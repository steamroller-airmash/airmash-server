/// Type-safe team identifier
#[derive(Copy, Clone, Eq, Hash, Debug, Default, PartialEq, From, Into, Constructor)]
pub struct Team(pub u16);

wrapper_serde_decl!(Team);

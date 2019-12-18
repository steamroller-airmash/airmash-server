/// A player ID
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, From, Into, Constructor)]
pub struct Player(pub u16);

wrapper_serde_decl!(Player);

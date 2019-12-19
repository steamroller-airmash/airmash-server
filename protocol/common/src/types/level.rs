/// Type-safe Level identifier
#[derive(
    Copy, Clone, Eq, Hash, Debug, Default, PartialEq, From, Into, Add, Sub, Constructor, AddAssign,
)]
pub struct Level(pub u8);

wrapper_serde_decl!(Level);

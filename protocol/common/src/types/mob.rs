/// A mob (missile, upgrade, or powerup) ID
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, From, Into, Constructor)]
pub struct Mob(pub u16);

wrapper_serde_decl!(Mob);

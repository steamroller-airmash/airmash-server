use airmash_server::*;
use specs::*;

#[derive(Copy, Clone, Debug, Component)]
pub struct TotalDamage(pub Health);

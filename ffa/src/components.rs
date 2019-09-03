use airmash_server::*;
use specs::prelude::*;

#[derive(Copy, Clone, Debug, Component)]
pub struct TotalDamage(pub Health);

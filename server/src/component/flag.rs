
use specs::*;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsPlayer;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsMissile;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsPowerup;

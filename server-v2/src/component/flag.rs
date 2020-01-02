use crate::ecs::NullStorage;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsPlayer;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsMissile;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsPowerup;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsSpectating;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsDead;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsChatThrottled;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsChatMuted;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct ForcePlayerUpdate;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsBoosting;

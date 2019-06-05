use specs::*;
use std::time::Instant;

#[derive(Clone, Debug, Copy)]
pub struct LastFrame(pub Instant);

#[derive(Clone, Debug, Copy)]
pub struct ThisFrame(pub Instant);

#[derive(Clone, Debug, Copy)]
pub struct StartTime(pub Instant);

#[derive(Clone, Debug, Copy, Component)]
pub struct LastUpdate(pub Instant);

#[derive(Clone, Debug, Copy, Component)]
pub struct LastShotTime(pub Instant);

#[derive(Clone, Debug, Copy, Component)]
pub struct MobSpawnTime(pub Instant);

#[derive(Clone, Debug, Copy, Component)]
pub struct MobDespawnTime(pub Instant);

#[derive(Clone, Debug, Copy, Component)]
pub struct SpectateStartTime(pub Instant);

#[derive(Clone, Debug, Copy, Component)]
pub struct LastKeyTime(pub Instant);

#[derive(Clone, Debug, Copy, Component)]
pub struct JoinTime(pub Instant);

#[derive(Clone, Debug, Copy, Component)]
pub struct LastStealthTime(pub Instant);

#[derive(Clone, Debug, Copy, Component)]
pub struct LastRepelTime(pub Instant);

#[derive(Clone, Debug, Copy, Component)]
pub struct LastRespawnTime(pub Instant);

#[derive(Clone, Debug, Copy, Component)]
pub struct LastScoreBoardTime(pub Instant);

macro_rules! impl_default {
	{
		$( $name:ident, )*
	} => {
		$(
			impl Default for $name {
				fn default() -> Self {
					$name(Instant::now())
				}
			}
		)*
	}
}

impl_default! {
	LastFrame,
	ThisFrame,
	StartTime,
	LastScoreBoardTime,
}

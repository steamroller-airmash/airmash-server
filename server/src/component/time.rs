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
pub struct SpectateStartTime(pub Instant);

#[derive(Clone, Debug, Copy, Component)]
pub struct LastKeyTime(pub Instant);

#[derive(Clone, Debug, Copy, Component)]
pub struct JoinTime(pub Instant);

#[derive(Clone, Debug, Copy, Component)]
pub struct LastStealthTime(pub Instant);

#[derive(Clone, Debug, Copy, Component)]
pub struct LastRepelTime(pub Instant);

impl Default for LastFrame {
	fn default() -> Self {
		LastFrame(Instant::now())
	}
}
impl Default for ThisFrame {
	fn default() -> Self {
		ThisFrame(Instant::now())
	}
}
impl Default for StartTime {
	fn default() -> Self {
		StartTime(Instant::now())
	}
}

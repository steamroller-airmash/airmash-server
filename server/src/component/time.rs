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
impl Default for LastUpdate {
	fn default() -> Self {
		LastUpdate(Instant::now())
	}
}
impl Default for MobSpawnTime {
	fn default() -> Self {
		MobSpawnTime(Instant::now())
	}
}
impl Default for SpectateStartTime {
	fn default() -> Self {
		SpectateStartTime(Instant::now())
	}
}

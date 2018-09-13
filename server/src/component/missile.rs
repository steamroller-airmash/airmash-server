use specs::*;
use types::*;

#[derive(Clone, Debug, Copy, Component)]
pub struct MissileTrajectory(pub Position, pub Distance);

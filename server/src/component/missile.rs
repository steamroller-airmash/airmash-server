use crate::types::*;
use specs::*;

#[derive(Clone, Debug, Copy, Component)]
pub struct MissileTrajectory(pub Position, pub Distance);

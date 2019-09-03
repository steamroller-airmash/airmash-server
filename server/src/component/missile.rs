use crate::types::*;
use specs::prelude::*;

#[derive(Clone, Debug, Copy, Component)]
pub struct MissileTrajectory(pub Position, pub Distance);

#![feature(raw, unsize, specialization, entry_insert, const_generics, try_trait)]
#![allow(incomplete_features)]

#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate log;
#[macro_use]
extern crate phf;
#[macro_use]
extern crate serde;

#[macro_use]
extern crate server_v2_macros;

extern crate self as server_v2;

#[macro_use]
mod macros;

mod data;

pub mod component;
pub mod ecs;
pub mod event;
pub mod resource;
pub mod server;
pub mod sysdata;
pub mod system;
pub mod util;

pub use airmash_protocol as protocol;

pub use crate::protocol::{
    AccelScalar, Distance, Energy, EnergyRegen, Health, HealthRegen, MobType as Mob,
    PlaneType as Plane, PlayerStatus, Position, Rotation, RotationRate, Speed, Team, Vector2,
    Velocity,
};

#[doc(hidden)]
pub mod __export {
    pub use shrev;
    pub use std;
}

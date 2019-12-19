#![feature(try_trait, specialization)]
// Temporary while server-v2 is being developed
#![allow(unused_parens, array_into_iter)]

// Crates with macros
#[macro_use]
extern crate log;
#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate airmash_server_macros;

// Regular Dependencies
extern crate airmash_protocol_v5 as protocol_v5;

// Public dependencies
pub extern crate airmash_protocol as protocol;

// Needs to be first because of macros
#[macro_use]
pub mod utils;

// Modules
mod dispatch;
mod handlers;
mod server;
mod status;

pub mod component;
pub mod consts;
pub mod systems;
pub mod task;
pub mod types;

pub use crate::dispatch::{Builder, SystemDeps, SystemInfo};
pub use crate::server::{AirmashServer, AirmashServerConfig};
pub use crate::types::{
	Accel, AccelScalar, Config, Connections, Distance, Energy, EnergyRegen, Flag, FutureDispatcher,
	GameMode, GameModeWriter, Health, HealthRegen, KeyState, Level, Mob, Name, Plane, Position,
	Score, Speed, Team, Time, Vector2, Velocity,
};
pub use airmash_server_macros::*;

#[doc(hidden)]
pub mod exported {
	pub use crate::dispatch::DEBUG_ADAPTER;
	#[cfg(features = "sentry")]
	pub use crate::utils::__internal_log_sentry_error;
}

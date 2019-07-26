#![feature(try_trait, async_await, box_into_pin)]

// Crates with macros
#[macro_use]
extern crate log;
#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate shred_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

// Regular Dependencies
extern crate airmash_protocol_v5 as protocol_v5;

// Public dependencies
pub extern crate airmash_protocol as protocol;

// Needed since it's used within the try_get! and log_none!
// macros. This means that everything within the server
// namespace can be accessed by doing ::airmash_server.
#[allow(unused)]
mod airmash_server {
	pub use crate::*;
}

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

pub use crate::server::{AirmashServer, AirmashServerConfig};

pub use crate::dispatch::{Builder, SystemDeps, SystemInfo};

pub use crate::types::{
	Accel, AccelScalar, Config, Connections, Distance, Energy, EnergyRegen, Flag, FutureDispatcher,
	GameMode, GameModeWriter, Health, HealthRegen, KeyState, Level, Mob, Name, Plane, Position,
	Score, Speed, Team, Time, Vector2, Velocity,
};

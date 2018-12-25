#![feature(try_from, try_trait)]

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
extern crate bounded_queue;
extern crate dimensioned;
extern crate fnv;
extern crate futures;
extern crate hashbrown;
extern crate hibitset;
extern crate htmlescape;
extern crate hyper;
extern crate rand;
extern crate rayon;
#[cfg(features = "sentry")]
extern crate sentry;
extern crate serde_json;
extern crate shred;
extern crate shrev;
extern crate special_map;
extern crate specs;
extern crate tokio;
extern crate uuid;
extern crate ws;

// Public dependencies
pub extern crate airmash_protocol as protocol;

// Needed since it's used within the try_get! and log_none!
// macros. This means that everything within the server
// namespace can be accessed by doing ::airmash_server.
#[allow(unused)]
mod airmash_server {
	pub use *;
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
pub mod types;

use protocol as airmash_protocol;

pub use server::{AirmashServer, AirmashServerConfig};

pub use dispatch::{Builder, SystemDeps, SystemInfo};

pub use types::{
	Accel, AccelScalar, Config, Connections, Distance, Energy, EnergyRegen, Flag, FutureDispatcher,
	GameMode, GameModeWriter, Health, HealthRegen, KeyState, Level, Mob, Name, Plane, Position,
	Score, Speed, Team, Time, Vector2, Velocity,
};

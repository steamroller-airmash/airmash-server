#![feature(optin_builtin_traits, try_from, try_trait)]

// Crates with macros
#[macro_use]
extern crate log;
extern crate dimensioned;
#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate shred_derive;
#[macro_use]
extern crate lazy_static;
extern crate serde;

// Regular Dependencies
extern crate bounded_queue;
extern crate fnv;
extern crate hibitset;
extern crate htmlescape;
extern crate hyper;
pub extern crate protocol_common;
extern crate protocol_v5;
extern crate rand;
extern crate rayon;
extern crate shred;
extern crate shrev;
extern crate simple_logger;
extern crate special_map;
extern crate specs;
extern crate tokio;
extern crate tokio_core;
extern crate uuid;
extern crate websocket;

use websocket::futures;

// Modules
mod builder;
mod dispatch;
mod handlers;
mod server;
mod status;
mod timeloop;
mod timers;
mod utils;

pub use protocol_common as protocol;

pub mod component;
pub mod consts;
pub mod systems;
pub mod types;

use protocol as airmash_protocol;

pub use builder::AirmashServer;

pub use dispatch::{Builder, SystemDeps, SystemInfo};

pub use types::{
	Accel, AccelScalar, Config, Connections, Distance, Energy, EnergyRegen, Flag, FutureDispatcher,
	GameMode, GameModeWriter, Health, HealthRegen, KeyState, Level, Mob, Name, Plane, Position,
	Score, Speed, Team, Time, Vector2, Velocity,
};

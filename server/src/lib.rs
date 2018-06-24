#![allow(dead_code)]
#![feature(optin_builtin_traits)]
#![feature(specialization)]

// Crates with macros
#[macro_use]
extern crate log;
#[macro_use]
extern crate dimensioned;
#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate shred_derive;
#[macro_use]
extern crate lazy_static;
#[cfg_attr(feature = "serde", macro_use)]
#[cfg(feature = "serde")]
extern crate serde;

// Regular Dependencies
extern crate bit_field;
extern crate ctrlc;
extern crate fnv;
extern crate htmlescape;
extern crate hyper;
extern crate phf;
extern crate rand;
extern crate rayon;
extern crate shred;
extern crate shrev;
extern crate simple_logger;
extern crate specs;
extern crate tokio;
extern crate tokio_core;
extern crate uuid;
extern crate websocket;

use websocket::futures;

// Modules
mod dispatch;
mod handlers;
mod metrics;
mod server;
mod timeloop;
mod timers;
mod utils;
mod builder;

pub mod protocol;
pub mod systems;
pub mod consts;
pub mod component;
pub mod types;

use protocol as airmash_protocol;

pub use websocket::OwnedMessage;

pub use builder::AirmashServer;
pub use metrics::MetricsHandler;

pub use dispatch::{Builder, SystemInfo, SystemDeps};

pub use types::{
	Position,
	Velocity,
	Distance,
	Speed,
	Accel,
	AccelScalar,
	Time,

	Plane,
	Mob,
	Team,
	Level,
	
	GameMode,	
	FutureDispatcher
};


#![allow(dead_code, unused_imports)]
#![feature(optin_builtin_traits)]

// Crates with macros
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate dimensioned;
#[macro_use]
extern crate specs_derive;

// Regular Dependencies
extern crate simple_logger;
extern crate rand;
extern crate specs;
extern crate fnv;
extern crate uuid;
extern crate websocket;
extern crate airmash_protocol;

use websocket::futures as futures;

// Modules
mod types;
mod systems;

use std::env;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver};

use specs::World;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    env::set_var("RUST_BACKTRACE", "1");

    let mut world = World::new();

    let (event_send, event_recv) = channel::<types::ConnectionEvent>();

    // Add resources
    info!(target: "server", "Setting up resources!");

    world.add_resource(types::Connections::new());
    world.add_resource(Mutex::new(event_recv));

    // Add systems
    info!(target: "server", "Setting up systems!");

    // Start websocket accessor
    info!(target: "server", "Starting websocket server!");

    // Start gameloop
    info!(target: "server", "Starting gameloop!");

    unimplemented!();
}

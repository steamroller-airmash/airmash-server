
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

use specs::World;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let mut world = World::new();

    // Add resources
    world.add_resource(types::Connections::new());

    unimplemented!();
}

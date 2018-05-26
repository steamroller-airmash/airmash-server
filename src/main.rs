
#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate dimensioned;

extern crate simple_logger;
extern crate rand;
extern crate specs;
extern crate fnv;
extern crate uuid;
extern crate websocket;
extern crate airmash_protocol;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    unimplemented!();
}

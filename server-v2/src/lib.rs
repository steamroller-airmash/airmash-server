#![feature(raw, unsize, specialization)]

#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate log;

pub mod component;
pub mod ecs;
pub mod resource;
pub mod server;
pub mod system;

pub use airmash_protocol as protocol;

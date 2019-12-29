#![feature(raw, unsize, specialization, entry_insert, const_generics)]
#![allow(incomplete_features)]

#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate log;

#[macro_use]
extern crate server_v2_macros;

extern crate self as server_v2;

pub mod component;
pub mod ecs;
pub mod resource;
pub mod server;
pub mod system;
pub mod sysdata;
pub mod util;
pub mod event;


pub use airmash_protocol as protocol;



#[doc(hidden)]
pub mod __export {
	pub use shrev;
	pub use std;
}

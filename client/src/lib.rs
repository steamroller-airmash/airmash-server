//!

#[macro_use]
extern crate log;

mod client;
mod config;
mod error;
mod game;

pub use self::client::{Client, Timeout};
pub use self::error::{ClientError, ClientResult};
pub use self::game::*;

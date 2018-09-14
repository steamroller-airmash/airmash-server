//! All new handlers should be declared in this module

mod register;

pub mod command;
pub mod game;
pub mod packet;

pub use self::register::register;

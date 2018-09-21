//! This module contains various interfaces that
//! don't really belong anywhere specific.
//! (not a system, handler, component, or type)

pub mod event_handler;
pub mod maybe_init;
pub mod timer;

mod tuple_array;

pub use self::event_handler::{EventHandler, EventHandlerTypeProvider};

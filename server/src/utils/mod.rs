//! Various utility types and macros.
//! 
//! These are mainly utility types that remove
//! boilerplate throughout the rest of the codebase 
//! but don't have (too) much in the way of dependencies
//! on the rest of the engine.

#[macro_use]
mod try_get;
mod tuple_array;

mod event_handler;
mod maybe_init;
pub mod timer;

pub use self::event_handler::{EventHandler, EventHandlerTypeProvider};
pub use self::maybe_init::MaybeInit;

// Needed within dispatch
pub(crate) use self::event_handler::EventHandlerWrapper;

// Internal logging hook
#[cfg(features = "sentry")]
pub use self::try_get::_internal_log_sentry_error;

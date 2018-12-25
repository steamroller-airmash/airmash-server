//! This module contains various interfaces that
//! don't really belong anywhere specific.
//! (not a system, handler, component, or type)

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

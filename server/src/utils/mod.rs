//! This module contains various interfaces that
//! don't really belong anywhere specific.
//! (not a system, handler, component, or type)

#[macro_use]
mod try_get;
mod tuple_array;

pub mod event_handler;
pub mod maybe_init;
pub mod timer;

#[allow(deprecated)]
pub use self::event_handler::{EventHandler, EventHandlerTypeProvider};
pub use self::maybe_init::MaybeInit;

// Internal logging hook
#[cfg(features = "sentry")]
pub use self::try_get::_internal_log_sentry_error;

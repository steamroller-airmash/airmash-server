//! Various utility types and macros.
//!
//! These are mainly utility types that remove
//! boilerplate throughout the rest of the codebase
//! but don't have (too) much in the way of dependencies
//! on the rest of the engine.

#[macro_use]
mod try_get;
#[macro_use]
mod system_info;
mod tuple_array;

mod event_handler;
mod kdtree;
mod maybe_init;
pub mod timer;

pub use self::event_handler::{EventHandler, EventHandlerTypeProvider};
pub use self::maybe_init::MaybeInit;

// Needed within dispatch
pub(crate) use self::event_handler::EventHandlerWrapper;

// Needed within collision
#[cfg(features = "kd-tree")]
pub(crate) use self::kdtree::KdTree;

// Internal logging hook
#[cfg(features = "sentry")]
pub use self::try_get::_internal_log_sentry_error;

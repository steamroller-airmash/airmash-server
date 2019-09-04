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

mod debug_entity;
mod event_deps;
mod event_handler;
mod history_storage;
mod kdtree;
mod maybe_init;
pub mod timer;

pub use self::debug_entity::DebugAdapter;
pub use self::event_deps::EventDeps;
pub use self::event_handler::{EventHandler, EventHandlerTypeProvider};
pub use self::history_storage::HistoricalStorageExt;
pub use self::maybe_init::MaybeInit;

// Needed within dispatch
pub(crate) use self::event_handler::EventHandlerWrapper;

// Needed within collision
pub(crate) use self::kdtree::KdTree;

// Internal logging hook
#[cfg(features = "sentry")]
pub use self::try_get::_internal_log_sentry_error;

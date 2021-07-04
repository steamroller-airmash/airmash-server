//! Events within the game.
//!
//! Each event here corresponds to some change of state within the game itself,
//! whether it be a player dying, to a new player entering the game, to a player
//! using the special, or a player going into spectate. For each of these
//! changes of game state an event will be dispatched.
//!
//! Event handlers are types which implement the [`EventHandler`] trait. Most of
//! the time these will be functions with the right signature however for cases
//! where the handler needs to keep custom internal state then any type which
//! implements the trait can be used.
//!
//! # Implementing an Event Handler
//! There are two ways to implement an event handler. The easiest and most
//! common way to do so is to use the [`handler`] attribute on a function with
//! the right signature. This will automatically register the handler with any
//! world that gets created provided it is initialized properly.
//!
//! Creating an event handler via the [`handler`] macro will look something like
//! this:
//! ```
//! # use airmash_server::{handler, AirmashWorld};
//! # struct InsertEventHere;
//! #[handler]
//! fn my_custom_handler(event: &InsertEventHere, game: &mut AirmashWorld) {
//!   // Do things here...
//! }
//! ```
//!
//! The other way to register a handler is to call [`AirmashWorld::register`]
//! with the handler of your choice. This is needed if you want to implement a
//! handler that is not a function or you don't want to register the event
//! handler unconditionally.
//!
//! To do this we must first define a handler and then later on register an
//! instance of it with whatever [`AirmashWorld`] instance we choose.
//! ```
//! # use airmash_server::{EventHandler, AirmashWorld, event::Frame};
//! #[derive(Default)]
//! pub struct CountFrames(usize);
//!
//! impl EventHandler<Frame> for CountFrames {
//!   fn on_event(&mut self, event: &Frame, game: &mut AirmashWorld) {
//!     self.0 += 1;
//!   }
//! }
//!
//! let mut game = AirmashWorld::with_test_defaults();
//! game.register(CountFrames::default());
//!
//! // ... run server
//! ```
//!
//! # Event Handler Priorities
//! All event handlers have an associated priority. A priority can be any
//! `i32` value. By default, handlers registered via [`handler`] have a priority
//! of 0 but this can be changed by passing the `priority` parameter to
//! the [`handler`] macro. A set of priorities has been provided within the
//! [`priority`] module and is used accordingly for the builtin server
//! handlers. More advanced use cases can use the full range of an `i32` to
//! achieve whatever ordering is needed.
//!
//! General guidelines for event handler priorities:
//! - If you don't need any ordering leave the priority as the default one
//!   ([`DEFAULT`]). Most event handlers should have this priority. By
//!   convention, almost all handlers which send packets to the client have this
//!   priority (with one exception).
//! - If you need to set some data on an entity before a packet is sent then use
//!   [`MEDIUM`] or [`HIGH`] priorities depending on your use-case.
//! - If you need to perform some cleanup at the end of event processing then
//!   use the [`CLEANUP`] priority. Any event handlers with lower priority will
//!   execute after the cleanup has been performed.
//!
//! Note that the order of execution of event handlers with the same priority
//! level is not guaranteed. They may be executed in any order depending on the
//! order they were registered. In order to guarantee correct order of execution
//! priorities must be used.
//!
//! ## Login Packet Priority
//! There is one exception to the guideline about event handlers which send
//! packets to the client all being at the [`DEFAULT`] priority. This is the
//! handler which is responsible for sending the initial [`Login`] packet on a
//! new connection. Since ordering at the same priority is not guaranteed and
//! the login packet must be send first on the connection it executes at a
//! higher priority than any other defautl event handler. Specifically, it has
//! priority [`LOGIN`]. This is relevant since any changes that need to be done
//! to player state before login occurs (e.g. setting teams or setting initial
//! position) must happen at an even higher prioity. For this use case a
//! [`PRE_LOGIN`] priority is provided for such initialization event handlers.
//!
//! # Event Handler Execution
//! Event handlers are executed by calling [`AirmashWorld::dispatch`]. This will
//! execute the registered event handlers for this event in decreasing order of
//! priority. Event handlers can also dispatch dependent events. However, only
//! one event can be active at a time. In the case where there is a nested event
//! that is dispatched it will be queued up and executed as soon as the current
//! event finishes executing.
//!
//! [`EventHandler`]: crate::EventHandler
//! [`handler`]: crate::handler
//! [`priority`]: crate::priority
//! [`AirmashWorld`]: crate::AirmashWorld
//! [`AirmashWorld::register`]: crate::AirmashWorld::register
//! [`AirmashWorld::dispatch`]: crate::AirmashWorld::dispatch
//! [`DEFAULT`]: crate::priority::DEFAULT
//! [`MEDIUM`]: crate::priority::MEDIUM
//! [`HIGH`]: crate::priority::HIGH
//! [`CLEANUP`]: crate::priority::CLEANUP
//! [`LOGIN`]: crate::priority::LOGIN
//! [`PRE_LOGIN`]: crate::priority::PRE_LOGIN
//! [`Login`]: crate::protocol::server::Login

use crate::protocol::KeyCode;
use airmash_protocol::Vector2;
use hecs::Entity;

mod collision;
mod missile;
mod packet;
mod player;

pub use self::collision::*;
pub use self::missile::*;
pub use self::packet::*;
pub use self::player::*;

/// Emitted during server startup.
///
/// This is useful for registering resources at startup if so desired.
#[derive(Copy, Clone, Debug, Default)]
pub struct ServerStartup;

/// Emitted at the very start of each frame.
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameStart;

/// Emitted at the very end of each frame, after timer events and zombie entity
/// cleanup.
///
/// Usually you want [`Frame`] instead.
#[derive(Copy, Clone, Debug, Default)]
pub struct FrameEnd;

/// Emitted near the end of the frame, before timer events and zombie entity
/// cleanup.
///
/// This is most likely the event you want if you need to perform arbitrary work
/// each frame.
#[derive(Copy, Clone, Debug, Default)]
pub struct Frame;

/// Emitted when an entity despawns but before the entity is deleted.
#[derive(Copy, Clone, Debug)]
pub struct EntityDespawn {
  pub entity: Entity,
}

/// Emitted when a new entity is created.
#[derive(Copy, Clone, Debug)]
pub struct EntitySpawn {
  pub entity: Entity,
}

/// An entity (player, missile, or mob) has left the horizon of another player.
#[derive(Copy, Clone, Debug)]
pub struct EventHorizon {
  pub player: Entity,
  /// The entity leaving/entering the horizon
  pub entity: Entity,
  /// The current state of the entity relative to the player
  pub in_horizon: bool,
}

/// Emitted whenever a player presses a key.
///
/// Note that this packet is only emitted if the player in question is alive. If
/// you need to listen to all key events then you'll need to install a handler
/// for [`PacketEvent<Key>`](crate::event::PacketEvent).
#[derive(Copy, Clone, Debug)]
pub struct KeyEvent {
  pub player: Entity,
  pub key: KeyCode,
  // True for pressed
  pub state: bool,
}

/// A player in a predator has started/stopped boosting
#[derive(Copy, Clone, Debug)]
pub struct EventBoost {
  /// The player to which the event pertains.
  pub player: Entity,
  /// Whether or not the player is now boosting.
  pub boosting: bool,
}

/// A player in a goliath has activated its special.
#[derive(Copy, Clone, Debug)]
pub struct EventRepel {
  pub player: Entity,
}

/// A player in a prowler has entered or left stealth.
#[derive(Copy, Clone, Debug)]
pub struct EventStealth {
  pub player: Entity,
  /// Whether or not the player is stealthed.
  pub stealthed: bool,
}

/// A player has run into a mountain.
#[derive(Copy, Clone, Debug)]
pub struct EventBounce {
  pub player: Entity,
  /// The old direction of the player before it bounced off the mountain.
  ///
  /// The current direction of the player is contained within the
  /// [`Velocity`](crate::component::Velocity) component.
  pub old_vel: Vector2<f32>,
}

/// A player's powerup has expired.
///
/// The powerup will be removed once the current event has completed.
#[derive(Copy, Clone, Debug)]
pub struct PowerupExpire {
  pub player: Entity,
}

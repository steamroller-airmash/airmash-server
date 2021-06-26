//! This module is a dedicated space for event handlers.
//!
//! These will all be implicitly registered via the #[handler] proc macro.
//! so the actual source of this module is just a list of other modules.

mod on_event_boost;
mod on_key_packet;
mod on_missile_despawn;
mod on_player_fire;
mod on_player_join;
mod on_event_horizon;
//! This module is a dedicated space for event handlers.
//!
//! These will all be implicitly registered via the #[handler] proc macro.
//! so the actual source of this module is just a list of other modules.

mod on_event_boost;
mod on_event_bounce;
mod on_event_horizon;
mod on_key_packet;
mod on_missile_despawn;
mod on_missile_terrain_collision;
mod on_player_fire;
mod on_player_join;
mod on_player_missile_collision;
mod on_player_killed;

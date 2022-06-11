//! This module is a dedicated space for event handlers.
//!
//! These will all be implicitly registered via the #[handler] proc macro.
//! so the actual source of this module is just a list of other modules.

mod chat;
mod on_command;
mod on_event_boost;
mod on_event_bounce;
mod on_event_horizon;
mod on_event_stealth;
mod on_key_packet;
mod on_missile_despawn;
mod on_missile_terrain_collision;
mod on_mob_despawn;
mod on_mob_spawn;
mod on_player_change_plane;
mod on_player_fire;
mod on_player_hit;
mod on_player_join;
mod on_player_killed;
mod on_player_leave;
mod on_player_missile_collision;
mod on_player_mob_collision;
mod on_player_powerup;
mod on_player_repel;
mod on_player_respawn;
mod on_player_score_update;
mod on_player_spawn;
mod on_player_spectate;

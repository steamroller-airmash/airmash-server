//! Systems and event handlers that are run every frame.
//!
//! These specify the behaviours of the server. Each one
//! handles a small part of the server functionality and
//! will either change things directly or publish events
//! which will cause other systems to do things.

pub mod admin;
pub mod builtin;
pub mod collision;
pub mod event;
pub mod missile;
pub mod packet;

mod energy_regen;
mod player_update;

pub use self::energy_regen::update_player_energy;
pub use self::player_update::update_positions;

pub use self::inner::register;

mod inner {
    use super::*;
    use crate::ecs::Builder;

    pub fn register(builder: &mut Builder) {
        builder
            .with_registrar(builtin::register)
            .with_registrar(packet::register)
            .with_registrar(event::register)
            .with_registrar(collision::register)
            .with_registrar(admin::register)
            .with_registrar(missile::register)
            //
            .with::<update_positions>()
            .with::<update_player_energy>();
    }
}

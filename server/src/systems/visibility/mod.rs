//! Systems for tracking which entities are visible to
//! each player.
//!

mod register;
mod gen_player_grid;

pub use self::gen_player_grid::GenPlayerGrid;

pub use self::register::register;

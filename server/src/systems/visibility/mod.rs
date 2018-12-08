//! Systems for tracking which entities are visible to
//! each player.
//!

mod gen_player_grid;
mod register;

pub use self::gen_player_grid::GenPlayerGrid;

pub use self::register::register;

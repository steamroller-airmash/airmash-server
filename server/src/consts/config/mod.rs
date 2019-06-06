mod hit_circles;
mod map_size;
mod powerup_radius;

pub use self::hit_circles::*;
pub use self::map_size::*;
pub use self::powerup_radius::*;

use std::time::Duration;

pub const SCORE_BOARD_DURATION: Duration = Duration::from_secs(5);

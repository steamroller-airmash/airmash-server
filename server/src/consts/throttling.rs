//! Constants that control when a player is throttled
//! or muted.

use std::time::Duration;

pub const THROTTLE_LIMIT: usize = 2;
pub const MUTE_LIMIT: usize = 15;

pub const THROTTLE_PERIOD: Duration = Duration::from_secs(4);
pub const MUTE_PERIOD: Duration = Duration::from_secs(60);

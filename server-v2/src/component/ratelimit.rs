//! Components for keeping track of
//! ratelimits. These should all be
//! wrappers around [`types::RateLimiter`][0].
//!
//! [0]: crate::types::RateLimiter

use crate::types::RateLimiter;
use specs::prelude::*;

#[derive(Clone, Debug, Component)]
pub struct ChatThrottleLimiter(pub RateLimiter);

#[derive(Clone, Debug, Component)]
pub struct ChatMuteLimiter(pub RateLimiter);

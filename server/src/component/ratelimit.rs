//! Components for keeping track of
//! ratelimits. These should all be
//! wrappers around [`types::RateLimiter`].

use specs::*;
use types::RateLimiter;

#[derive(Clone, Debug, Component)]
pub struct ChatThrottleLimiter(pub RateLimiter);

#[derive(Clone, Debug, Component)]
pub struct ChatMuteLimiter(pub RateLimiter);

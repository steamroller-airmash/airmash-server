//! Resources that are setup by the server implementation
//! and can be used to control the event loop.

use std::time::Instant;

/// Once set, the server will shutdown immediately
/// after the current frame is done.
#[derive(Debug)]
pub struct ShutdownFlag(bool);

impl ShutdownFlag {
    pub(crate) fn new() -> Self {
        Self(false)
    }

    /// Whether the server will shutdown after the current frame.
    pub fn value(&self) -> bool {
        self.0
    }

    /// Set the shutdown flag.
    pub fn shutdown(&mut self) {
        self.0 = true;
    }
}

/// The number of players currently online.
///
/// This is used to expose the player count endpoint.
#[derive(Copy, Clone, Debug, Default)]
pub struct PlayerCount(pub usize);

/// The time at which the current frame was scheduled.
#[derive(Copy, Clone, Debug)]
pub struct CurrentFrame(pub Instant);

/// The time at which the last frame was scheduled.
///
/// For the very first frame this is some arbitrary
/// time that is before the current frame time.
#[derive(Copy, Clone, Debug)]
pub struct LastFrame(pub Instant);

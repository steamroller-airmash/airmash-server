//! Utility accessors for components
//! that are commonly used (read-only)
//! together. Writes to these components
//! must still be done individually

mod isalive;
mod clock;

pub use self::isalive::IsAlive;
pub use self::clock::ReadClock;

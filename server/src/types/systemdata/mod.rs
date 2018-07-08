//! Utility accessors for components
//! that are commonly used (read-only)
//! together. Writes to these components
//! must still be done individually

mod clock;
mod isalive;

pub use self::clock::ReadClock;
pub use self::isalive::IsAlive;

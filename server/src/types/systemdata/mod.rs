//! Utility accessors for components 
//! that are commonly used (read-only)
//! together. Writes to these components
//! must still be done individually

mod isalive;

pub use self::isalive::IsAlive;

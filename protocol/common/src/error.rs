//! All error types for this crate.

/// Attempted to convert an enum from a value but
/// the value didn't map to any possible enum value.
pub struct EnumValueOutOfRangeError<T>(pub T);

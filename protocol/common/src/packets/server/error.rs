use crate::enums::ErrorType;

/// The client has carried out an invalid action,
/// been ratelimited, or is banned.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Error {
	pub error: ErrorType,
}

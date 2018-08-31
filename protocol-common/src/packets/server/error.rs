use enums::ErrorType;

/// The client has carried out an invalid action,
/// been ratelimited, or is banned.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct Error {
	pub error: ErrorType,
}

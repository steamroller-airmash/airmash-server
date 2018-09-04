use enums::KeyCode;

/// Send a key update for the client.
///
/// Notes:
/// - `seq` should be monotonically increasing
///   with every key press.
/// - `state`: `true` -> pressed, `false` -> released.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Key {
	pub seq: u32,
	pub key: KeyCode,
	pub state: bool,
}

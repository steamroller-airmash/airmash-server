use crate::enums::ServerCustomType;

/// End of game packet for CTF and BTR.
///
/// # CTF
/// In CTF, the data of this packet contains
/// a JSON string with 3 fields.
///
/// - `w`: The id of the winning team.
/// - `b`: The bounty given to each player
/// of the winning team.
/// - `t`: The time (in seconds) that the
/// banner should remain on screen before
/// closing (unless closed by the player.)
///
/// # BTR
/// In BTR, the data of this packet contains
/// a JSON string with 5 fields.
///
/// - `p`: The name of the winning player.
/// - `f`: The flag id of the winning player.
/// - `b`: The bounty given to the winning player.
/// - `k`: The number of kills that the winning player has.
/// - `t`: The time (in seconds) that the banner should
///        remain on the screen before closing (unless
///        closed by the player.)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ServerCustom {
	#[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub ty: ServerCustomType,
	pub data: String,
}

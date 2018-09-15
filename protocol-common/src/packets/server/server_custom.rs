use enums::ServerCustomType;

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
/// closing (unless closed by the player).
///
/// # BTR
/// TODO
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ServerCustom {
	#[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub ty: ServerCustomType,
	pub data: String,
}

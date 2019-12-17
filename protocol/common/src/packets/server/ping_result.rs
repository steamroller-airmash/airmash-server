/// Resulting ping data sent back from the server.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PingResult {
	pub ping: u16,
	#[cfg_attr(feature = "serde", serde(rename = "playersTotal"))]
	pub players_total: u32,
	#[cfg_attr(feature = "serde", serde(rename = "playersGame"))]
	pub players_game: u32,
}

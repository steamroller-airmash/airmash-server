/// Resulting ping data sent back from the server.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PingResult {
	pub ping: u16,
	#[serde(rename = "playersTotal")]
	pub players_total: u32,
	#[serde(rename = "playersGame")]
	pub players_game: u32,
}

/// Info on the number of players currently alive
#[derive(Copy, Clone, Debug)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct GamePlayersAlive {
	pub players: u16,
}

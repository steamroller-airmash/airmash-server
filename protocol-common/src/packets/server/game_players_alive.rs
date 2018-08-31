/// Info on the number of players currently alive
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct GamePlayersAlive {
	pub players: u16,
}

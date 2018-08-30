/// Send a whisper to another player.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Whisper {
	pub id: u16,
	pub text: String,
}

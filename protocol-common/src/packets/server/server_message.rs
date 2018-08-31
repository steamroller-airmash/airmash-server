use enums::ServerMessageType;

/// Server banned message
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerMessage {
	#[serde(rename = "type")]
	pub ty: ServerMessageType,
	// TODO: Make this a duration?
	pub duration: u32,
	pub text: String,
}

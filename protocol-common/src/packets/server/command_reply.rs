use enums::CommandReplyType;

/// Reply to a [`Command`](../client/struct.command.html).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandReply {
	#[serde(rename = "type")]
	pub ty: CommandReplyType,
	pub text: String,
}

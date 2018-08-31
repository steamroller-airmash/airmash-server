use enums::CommandReplyType;

/// Reply to a [`Command`](../client/struct.command.html).
#[derive(Clone, Debug)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct CommandReply {
	#[cfg_attr(features = "serde", serde(rename = "type"))]
	pub ty: CommandReplyType,
	pub text: String,
}

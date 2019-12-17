use crate::enums::CommandReplyType;

/// Reply to a [`Command`](../client/struct.command.html).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommandReply {
	#[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub ty: CommandReplyType,
	pub text: String,
}

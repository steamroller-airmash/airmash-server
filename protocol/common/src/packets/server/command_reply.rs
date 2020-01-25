use crate::enums::CommandReplyType;

use std::borrow::Cow;

/// Reply to a [`Command`](../client/struct.command.html).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommandReply<'data> {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub ty: CommandReplyType,
    pub text: Cow<'data, str>,
}

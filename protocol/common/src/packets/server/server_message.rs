use crate::enums::ServerMessageType;

use std::borrow::Cow;

/// Any general server message
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ServerMessage<'data> {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub ty: ServerMessageType,
    // TODO: Make this a duration?
    pub duration: u32,
    pub text: Cow<'data, str>,
}

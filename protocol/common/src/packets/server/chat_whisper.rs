use crate::types::Player;

use std::borrow::Cow;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatWhisper<'data> {
    pub from: Player,
    pub to: Player,
    pub text: Cow<'data, str>,
}

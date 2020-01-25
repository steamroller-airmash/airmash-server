use crate::types::Player;

use std::borrow::Cow;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatSay<'data> {
    pub id: Player,
    pub text: Cow<'data, str>,
}

use crate::types::Player;

use std::borrow::Cow;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatPublic<'data> {
    pub id: Player,
    pub text: Cow<'data, str>,
}

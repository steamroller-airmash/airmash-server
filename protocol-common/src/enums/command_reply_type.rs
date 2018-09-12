use specs::DenseVecStorage;

use std::convert::From;

/// TODO: Reverse engineer
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "specs", derive(Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CommandReplyType {
	ShowInConsole = 0,
	/// Technically this should be any value other than 0,
	/// the from integer implementation for this enum deals
	/// with that.
	ShowInPopup = 1,
}

macro_rules! decl_from {
	($ty:ty) => {
		impl From<$ty> for CommandReplyType {
			fn from(v: $ty) -> Self {
				match v {
					0 => CommandReplyType::ShowInConsole,
					_ => CommandReplyType::ShowInPopup,
				}
			}
		}

		impl From<CommandReplyType> for $ty {
			fn from(v: CommandReplyType) -> $ty {
				v as $ty
			}
		}
	};
}

decl_from!(u8);
decl_from!(u16);
decl_from!(u32);

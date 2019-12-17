#[cfg(feature = "specs")]
use specs::{Component, DenseVecStorage};

/// Game Type.
///
/// Hopefully self explanatory, used to indicate to
/// the client which game is being played. The client
/// uses this to decide on player colouring and
/// whether or not to show the flags in-game.
/// It will also correspond with the type of detailed
/// score ([`ScoreDetailedFFA`][0], [`ScoreDetailedCTF`][1],
/// or [`ScoreDetailedBTR`][2]) that the client expects
/// to receive.
///
/// Used in:
/// - TODO
///
/// [0]: server/struct.ScoreDetailedFFA.html
/// [1]: server/struct.ScoreDetailedCTF.html
/// [2]: server/struct.ScoreDetailedBTR.html
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Primitive)]
#[cfg_attr(feature = "specs", derive(Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GameType {
	FFA = 1,
	CTF = 2,
	BTR = 3,
}

impl_try_from2!(GameType);

impl Default for GameType {
	fn default() -> Self {
		GameType::FFA
	}
}

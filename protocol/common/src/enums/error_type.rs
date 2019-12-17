#[cfg(feature = "specs")]
use specs::{Component, DenseVecStorage};

/// All error codes that can be sent to the client.
///
/// These are all server errors that the vanilla AIRMASH
/// client (and the current STARMASH client) understands.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Primitive)]
#[cfg_attr(feature = "specs", derive(Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ErrorType {
	DisconnectedForPacketFlooding = 1,
	BannedForPacketFlooding = 2,
	Banned = 3,
	IdleRequiredBeforeRespawn = 5,
	AfkTimeout = 6,
	Kicked = 7,
	InvalidLogin = 8,
	IncorrectProtocolLevel = 9,
	AccountBanned = 10,
	AccountAlreadyLoggedIn = 11,
	NoRespawnInBTR = 12,
	IdleRequiredBeforeSpectate = 13,
	NotEnoughUpgrades = 20,
	ChatThrottled = 30,
	FlagChangeThrottled = 31,
	UnknownCommand = 100,
}

impl_try_from2!(ErrorType);

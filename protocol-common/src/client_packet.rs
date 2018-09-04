use client::*;

/// All possible client packets.
///
/// This contains all valid packets that
/// the client can send to the server
/// (in the current version of the airmash
/// protocol).
///
/// Some packets don't contain any data, these
/// packets do not have an associated struct
/// and as such are just empty variants within
/// this enum.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ClientPacket {
	Login(Login),
	Backup(Backup),
	Horizon(Horizon),
	Ack,
	Pong(Pong),
	Key(Key),
	Command(Command),
	ScoreDetailed,
	Chat(Chat),
	TeamChat(TeamChat),
	Whisper(Whisper),
	Say(Say),
	VoteMute(VoteMute),
	LocalPing(LocalPing),
}

macro_rules! impl_from_newtype {
	($type:tt) => {
		impl_from_newtype_inner!(ClientPacket, $type);
	};
}

macro_rules! impl_from_empty {
	($type:tt) => {
		impl_from_empty_inner!(ClientPacket, $type);
	};
}

impl_from_newtype!(Login);
impl_from_newtype!(Backup);
impl_from_newtype!(Horizon);
impl_from_newtype!(Pong);
impl_from_newtype!(Key);
impl_from_newtype!(Command);
impl_from_newtype!(Chat);
impl_from_newtype!(TeamChat);
impl_from_newtype!(Whisper);
impl_from_newtype!(Say);
impl_from_newtype!(VoteMute);
impl_from_newtype!(LocalPing);

impl_from_empty!(Ack);
impl_from_empty!(ScoreDetailed);

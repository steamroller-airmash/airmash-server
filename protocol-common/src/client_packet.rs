
use client::*;

/// All possible client packets.
///
/// This contains all valid packets that
/// the client can send to the server
/// (in the current version of the airmash
/// protocol). It can be serialized and
/// deserialized to/from byte buffers
/// using [`to_bytes`](fn.to_bytes.html)
/// and [`from_bytes`](fn.from_bytes.html).
///
/// Some packets don't contain any data, these
/// packets do not have an associated struct
/// and as such are just empty variants within
/// this enum.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientPacket {
	Login(ClientLogin),
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

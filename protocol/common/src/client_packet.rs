use crate::client::*;

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
///
/// The [`From`][0] trait has been implemented
/// for all the structs that correspond to the
/// variants of this enum. This means that instead
/// of directly constructing an instance of
/// `ClientPacket`, [`into()`][1] can be called
/// instead.
///
/// [0]: https://doc.rust-lang.org/std/convert/trait.From.html
/// [1]: https://doc.rust-lang.org/std/convert/trait.Into.html#tymethod.into
#[derive(Clone, Debug, From)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ClientPacket<'data> {
    Login(Login<'data>),
    Backup(Backup<'data>),
    Horizon(Horizon),
    Ack,
    Pong(Pong),
    Key(Key),
    Command(Command<'data>),
    ScoreDetailed,
    Chat(Chat<'data>),
    TeamChat(TeamChat<'data>),
    Whisper(Whisper<'data>),
    Say(Say<'data>),
    VoteMute(VoteMute),
    LocalPing(LocalPing),
}

impl From<Ack> for ClientPacket<'_> {
    fn from(_: Ack) -> Self {
        Self::Ack
    }
}

impl From<ScoreDetailed> for ClientPacket<'_> {
    fn from(_: ScoreDetailed) -> Self {
        Self::ScoreDetailed
    }
}

//! Packets that have no data associated with them.

/// Acknowledge successful receipt of a
/// [`Backup`][0] packet.
///
/// [0]: ../client/struct.backup.html
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Backup;

/// TODO: Figure out what this does.
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ack;

/// The current player has been votemuted.
///
/// This happens after enough players have sent
/// a [`VoteMute`][0] packet to the server.
///
/// [0]: ../client/struct.VoteMute.html
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatVoteMuted;

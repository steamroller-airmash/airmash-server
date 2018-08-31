//! Packets that have no data associated with them.

/// Acknowledge successful receipt of a
/// [`Backup`][0] packet.
///
/// [0]: ../client/struct.backup.html
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Backup;

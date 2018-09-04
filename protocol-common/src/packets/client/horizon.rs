/// Packet to tell the server to resize the horizon
/// for the client.
///
/// In theory this should expand the visible range
/// for the client, in practice the official server
/// appears to ignore these packets.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Horizon {
	pub horizon_x: u16,
	pub horizon_y: u16,
}

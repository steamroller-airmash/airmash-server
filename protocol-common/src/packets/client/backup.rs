/// Opening packet for opening a second
/// server connection for the same client.
///
/// This packet is used to allow for
/// multiple websocket connections to
/// the airmash server. To open a second
/// connection, open a websocket connection
/// to the server, then send this packet
/// as the first packet instead of sending
/// [`Login`](struct.login.html). The server
/// will respond to client packets sent through
/// this channel, allowing for some reduction
/// in head of line blocking.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Backup {
	pub token: String,
}

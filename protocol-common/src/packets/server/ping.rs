/// A ping request by the server.
///
/// All clients must respond with a
/// [`Pong`](../client/struct.pong.html)
/// with `num` set to the same value
/// as this packet. If a client does
/// not do this, the client will be
/// disconnected by the server.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ping {
	pub clock: u32,
	pub num: u32,
}

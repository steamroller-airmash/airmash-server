/// Initial packet sent to log in to the server.
///
/// This sent to the server when the player
/// first joins
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Login {
	/// The current protocol version.
	/// Should always be 5 as of the
	/// writing of this documentation.
	pub protocol: u8,
	/// The name that the player wishes to
	/// be called on the server. The actual
	/// name of the player given by the server
	/// will be in the
	/// [`Login`](../server/struct.login.html)
	/// packet returned by the server.
	pub name: String,
	/// A session token for the current player.
	/// This session token is the way that a
	/// player would log in to the server. If
	/// the player does not wish to be logged
	/// on to the server then a session token
	/// of `"none"` will suffice.
	pub session: String,
	/// Should set the size of the horizon beyond
	/// which game updates (missile updates and
	/// player updates) are not sent to the client.
	/// In practice, this doesn't appear to be
	/// used by the official server.
	pub horizon_x: u16,
	/// Same as `horizon_x` but in the y direction.
	pub horizon_y: u16,
	/// The desired flag of the player. This should
	/// be the ISO-2 country code corresponding to
	/// the flag that the player wishes to take. It
	/// may also be one of the special flag codes
	/// for non-country flags.
	///
	/// If the flag code passed in is not one of the
	/// ones for which there is a known (to the server)
	/// flag, then the player will be assigned to
	/// UN flag (in the official server).
	pub flag: String,
}

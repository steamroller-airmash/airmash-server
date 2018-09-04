/// A free form command to be sent to the server.
/// This is used for changing flags, respawning,
/// spectating players, and selecting upgrades.
///
/// # Changing a flag
/// ```
/// # extern crate airmash_protocol;
/// # use airmash_protocol::client::Command;
/// # fn main() {
/// let cmd = Command {
///     com: "flag".to_string(),
///     // Set to desired flag code,
///     // unknown will result in UN flag.
///     // Here we will set to the UN flag.
///     data: "XX".to_string()
/// };
///
/// // Serialize and send to server here...
/// # }
/// ```
///
/// # Respawning as a plane
/// ```
/// # extern crate airmash_protocol;
/// # use airmash_protocol::client::Command;
/// # fn main() {
/// let cmd = Command {
///     com: "respawn".to_string(),
///     // Choose the plane type here,
///     // each type is associated with
///     // an integer. Here we will pick
///     // predator.
///     data: "1".to_string()
/// };
///
/// // Serialize and send to server here...
/// # }
/// ```
///
/// # Selecting Upgrades
/// ```
/// # extern crate airmash_protocol;
/// # use airmash_protocol::client::Command;
/// # fn main() {
/// let cmd = Command {
///     com: "upgrade".to_string(),
///     // Choose upgrade type here.
///     // Here speed should be 1.
///     data: "1".to_string()
/// };
///
/// // Serialize and send to server here...
/// # }
/// ```
///
/// # Going into spectate or spectating a different player
/// ```
/// # extern crate airmash_protocol;
/// # use airmash_protocol::client::Command;
/// # fn main() {
/// let cmd = Command {
///     com: "spectate".to_string(),
///     // This can either be a player id, or
///     // one of -1, -2, or -3. -3 will force
///     // the player to go into spectate,
///     // -1 switches focus to the next player,
///     // and -2 switches focus to the previous
///     // player. Here we will force the player
///     // to go into spectate.
///     data: "-3".to_string()
/// };
///
/// // Serialize and send to server here...
/// # }
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Command {
	/// The command to send to the server. The
	/// official server recognizes the commands
	/// `"spectate"`, `"upgrade"`, `"flag"`, and
	/// `"respawn"`.
	pub com: String,
	/// The data associated with the command,
	/// value values epend on the given command.
	pub data: String,
}

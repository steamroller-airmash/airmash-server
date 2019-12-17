#[cfg(feature = "specs")]
use specs::{Component, DenseVecStorage};

/// Upgrade info that a client needs to know about to
/// calculate movement. This also includes the shielded
/// state of the player.
///
/// Note that since a player should never have more than
/// 5 upgrades on the official server, `protocol-v5` can
/// only represent amounts of speed upgrades in the range
/// 0 to 7.
///
/// Used in:
/// - [`Login`](server/struct.Login.html), specifically
///   [`LoginPlayer`](server/struct.LoginPlayer.html)
/// - [`PlayerUpdate`](server/struct.PlayerUpdate.html)
/// - [`PlayerRespawn`](server/struct.PlayerRespawn.html)
/// - [`PlayerUpgrade`](server/struct.PlayerUpgrade.html)
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "specs", derive(Component))]
pub struct Upgrades {
	/// The number of speed upgrades that the player currently
	/// has equipped.
	///
	/// Note that only the first 3 bits of this are used
	/// in protocol-v5. Any values greater than 7 will be
	/// mangled.
	pub speed: u8,
	/// Whether the player has a shield.
	///
	/// While both this and [`inferno`][0] can be
	/// set at the same time, that doesn't make
	/// sense within the framework of the game.
	///
	/// [0]: #structfield.inferno
	pub shield: bool,
	/// Whether the player has an inferno.
	///
	/// While both this and [`shield`][0] can be
	/// set at the same time, that doesn't make
	/// sense within the framework of the game.
	///
	/// [0]: #structfield.shield
	pub inferno: bool,
}

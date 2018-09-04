#[cfg(feature = "specs")]
use specs::DenseVecStorage;

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
	/// Note that only the first 3 bits of this are used
	/// in protocol-v5
	pub speed: u8,
	pub shield: bool,
	pub inferno: bool,
}

/// Upgrade info that a client needs to know about to
/// calculate movement. This also includes the shielded
/// state of the player.
///
/// Used in:
/// - [`Login`](server/struct.Login.html), specifically
///   [`LoginPlayer`](server/struct.LoginPlayer.html)
/// - [`PlayerUpdate`](server/struct.PlayerUpdate.html)
/// - [`PlayerRespawn`](server/struct.PlayerRespawn.html)
/// - [`PlayerUpgrade`](server/struct.PlayerUpgrade.html)
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct Upgrades {
	/// Note that only the first 3 bits of this are used
	/// in protocol-v5
	pub speed: u8,
	pub shield: bool,
	pub inferno: bool,
}

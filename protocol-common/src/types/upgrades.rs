#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct Upgrades {
	/// Note that only the first 3 bits of this are used
	/// in protocol-v5
	pub speed: u8,
	pub shield: bool,
	pub inferno: bool,
}

use specs::*;

#[derive(Default, Clone, Copy, Debug, Component)]
pub struct Upgrades {
	pub speed: u8,
	pub defense: u8,
	pub energy: u8,
	pub missile: u8,
	pub unused: u16,
}

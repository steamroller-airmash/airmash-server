
use specs::*;

#[derive(Default, Clone, Copy, Debug, Component)]
pub struct Powerups {
	pub inferno: bool,
	pub shield: bool
}

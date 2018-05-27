
use specs::*;

#[derive(Component, Default, Clone)]
pub struct KeyState {
	pub up: bool,
	pub down: bool,
	pub left: bool,
	pub right: bool,
	pub fire: bool,
	pub special: bool
}

#[derive(Component)]
pub struct KeyStateUpdated;

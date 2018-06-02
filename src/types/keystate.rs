use specs::*;

use airmash_protocol::{PlaneType, ServerKeyState};
use types::Plane;

#[derive(Default, Clone, Debug)]
pub struct KeyState {
	pub up: bool,
	pub down: bool,
	pub left: bool,
	pub right: bool,
	pub fire: bool,
	pub special: bool,
	// This might not be the best place to
	// keep these, can be moved later if
	// necessary
	pub stealthed: bool,
	pub flagspeed: bool,
}

impl KeyState {
	pub fn boost(&self, plane: &Plane) -> bool {
		*plane == PlaneType::Predator && self.special
	}
	pub fn strafe(&self, plane: &Plane) -> bool {
		*plane == PlaneType::Mohawk && self.special
	}

	pub fn to_server(&self, plane: &Plane) -> ServerKeyState {
		use airmash_protocol::ServerKeyState as Key;
		let mut state = ServerKeyState(0);

		state.set(Key::UP, self.up);
		state.set(Key::DOWN, self.down);
		state.set(Key::LEFT, self.left);
		state.set(Key::RIGHT, self.right);
		state.set(Key::BOOST, self.boost(plane));
		state.set(Key::STRAFE, self.strafe(plane));
		state.set(Key::STEALTH, self.stealthed);
		state.set(Key::FLAGSPEED, self.flagspeed);

		state
	}
}

impl Component for KeyState {
	type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

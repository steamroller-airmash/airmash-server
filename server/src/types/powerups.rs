use specs::*;

use crate::protocol::PowerupType;

use std::time::Instant;

#[derive(Copy, Clone, Debug, Component)]
pub struct Powerups {
	pub ty: PowerupType,
	pub end_time: Instant,
}

pub trait PowerupExt {
	fn shield(&self) -> bool;
	fn inferno(&self) -> bool;
}

impl PowerupExt for Powerups {
	fn shield(&self) -> bool {
		self.ty == PowerupType::Shield
	}
	fn inferno(&self) -> bool {
		self.ty == PowerupType::Inferno
	}
}

impl PowerupExt for Option<Powerups> {
	fn shield(&self) -> bool {
		self.map(|details| details.shield()).unwrap_or(false)
	}

	fn inferno(&self) -> bool {
		self.map(|details| details.inferno()).unwrap_or(false)
	}
}

impl<'a> PowerupExt for Option<&'a Powerups> {
	fn shield(&self) -> bool {
		self.map(|x| x.shield()).unwrap_or(false)
	}
	fn inferno(&self) -> bool {
		self.map(|x| x.inferno()).unwrap_or(false)
	}
}

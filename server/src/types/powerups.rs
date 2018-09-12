use specs::*;

use protocol::PowerupType;

use std::time::Instant;

#[derive(Copy, Clone, Debug)]
pub struct PowerupDetails {
	pub ty: PowerupType,
	pub end_time: Instant,
}

#[derive(Default, Clone, Copy, Debug, Component)]
pub struct Powerups {
	pub details: Option<PowerupDetails>,
}

impl Powerups {
	pub fn shield(&self) -> bool {
		self.details
			.map(|details| details.ty == PowerupType::Shield)
			.unwrap_or(false)
	}

	pub fn inferno(&self) -> bool {
		self.details
			.map(|details| details.ty == PowerupType::Inferno)
			.unwrap_or(false)
	}
}

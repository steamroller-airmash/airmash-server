
use specs::prelude::*;
use types::*;

use component::time::{LastFrame, ThisFrame};
use component::flag::IsPlayer;

pub struct EnergyRegenSystem;

#[derive(SystemData)]
pub struct EnergyRegenSystemData<'a> {
	pub lastframe: Read<'a, LastFrame>,
	pub thisframe: Read<'a, ThisFrame>,
	pub config: Read<'a, Config>,

	pub energy: WriteStorage<'a, Energy>,
	pub plane: ReadStorage<'a, Plane>
}

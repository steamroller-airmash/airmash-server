use specs::prelude::*;
use types::*;

use component::flag::IsPlayer;
use component::time::{LastFrame, ThisFrame};

pub struct EnergyRegenSystem;

#[derive(SystemData)]
pub struct EnergyRegenSystemData<'a> {
	pub lastframe: Read<'a, LastFrame>,
	pub thisframe: Read<'a, ThisFrame>,
	pub config: Read<'a, Config>,

	pub energy: WriteStorage<'a, Energy>,
	pub plane: ReadStorage<'a, Plane>,
	pub flag: ReadStorage<'a, IsPlayer>,
	pub upgrades: ReadStorage<'a, Upgrades>,
}

impl<'a> System<'a> for EnergyRegenSystem {
	type SystemData = EnergyRegenSystemData<'a>;

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			lastframe,
			thisframe,
			config,
			mut energy,
			plane,
			flag,
			upgrades,
		} = data;

		let dt = Time::new((thisframe.0 - lastframe.0).subsec_nanos() as f32 * (60.0 / 1.0e9));

		(&mut energy, &plane, &flag, &upgrades)
			.join()
			.map(|(energy, plane, _, upgrades)| {
				let regen = config.planes[*plane].energy_regen;
				let mult = config.upgrades.energy.factor[upgrades.energy as usize];

				(energy, regen * mult)
			})
			.for_each(|(energy, regen)| {
				let val: Energy = *energy + regen * dt;

				*energy = Energy::new(val.inner().min(1.0));
			});
	}
}

use super::missile::MissileFireHandler;
use dispatch::SystemInfo;

impl SystemInfo for EnergyRegenSystem {
	type Dependencies = MissileFireHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self {}
	}
}

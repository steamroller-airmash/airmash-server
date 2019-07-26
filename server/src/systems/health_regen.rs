use crate::types::*;
use specs::*;

use crate::component::flag::IsPlayer;
use crate::component::time::{LastFrame, ThisFrame};

use crate::dispatch::SystemInfo;
use crate::systems::missile::MissileHit;

pub struct HealthRegenSystem;

#[derive(SystemData)]
pub struct HealthRegenSystemData<'a> {
	pub flag: ReadStorage<'a, IsPlayer>,
	pub health: WriteStorage<'a, Health>,
	pub regen: ReadStorage<'a, HealthRegen>,

	pub lastframe: Read<'a, LastFrame>,
	pub thisframe: Read<'a, ThisFrame>,
}

impl<'a> System<'a> for HealthRegenSystem {
	type SystemData = HealthRegenSystemData<'a>;

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			flag,
			mut health,
			regen,
			thisframe,
			lastframe,
		} = data;

		let dt = Time::new((thisframe.0 - lastframe.0).subsec_nanos() as f32 * (60.0 / 1.0e9));

		(&flag, &mut health, &regen)
			.join()
			.for_each(|(_, health, regen)| {
				// Make sure to get units right
				let newhealth: Health = *health + *regen * dt;

				// Units don't support max or min, have to unwrap
				*health = Health::new(newhealth.inner().min(1.0).max(0.0));
			});
	}
}

impl SystemInfo for HealthRegenSystem {
	type Dependencies = MissileHit;

	fn new() -> Self {
		Self {}
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

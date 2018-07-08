use specs::*;

use types::*;

use SystemInfo;

use super::InitTraits;

use component::channel::*;

pub struct InitState {
	reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct InitStateData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,
	pub config: Read<'a, Config>,

	pub plane: ReadStorage<'a, Plane>,
	pub energy: WriteStorage<'a, Energy>,
	pub health: WriteStorage<'a, Health>,
	pub keystate: WriteStorage<'a, KeyState>,
	pub powerups: WriteStorage<'a, Powerups>,
	pub upgrades: WriteStorage<'a, Upgrades>,
	pub energy_regen: WriteStorage<'a, EnergyRegen>,
	pub health_regen: WriteStorage<'a, HealthRegen>,
}

impl<'a> System<'a> for InitState {
	type SystemData = InitStateData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let ref info = data.config.planes[*data.plane.get(evt.id).unwrap()];
			let energy_regen = info.energy_regen;
			let health_regen = info.health_regen;

			data.energy.insert(evt.id, Energy::new(1.0)).unwrap();
			data.health.insert(evt.id, Health::new(1.0)).unwrap();
			data.keystate.insert(evt.id, KeyState::default()).unwrap();
			data.powerups.insert(evt.id, Powerups::default()).unwrap();
			data.upgrades.insert(evt.id, Upgrades::default()).unwrap();
			data.energy_regen.insert(evt.id, energy_regen).unwrap();
			data.health_regen.insert(evt.id, health_regen).unwrap();
		}
	}
}

impl SystemInfo for InitState {
	type Dependencies = (InitTraits);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

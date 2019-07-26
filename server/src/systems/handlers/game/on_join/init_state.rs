use specs::*;

use super::InitTraits;
use crate::types::*;
use crate::SystemInfo;

use crate::component::event::*;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct InitState;

#[derive(SystemData)]
pub struct InitStateData<'a> {
	config: Read<'a, Config>,

	plane: ReadStorage<'a, Plane>,
	energy: WriteStorage<'a, Energy>,
	health: WriteStorage<'a, Health>,
	keystate: WriteStorage<'a, KeyState>,
	upgrades: WriteStorage<'a, Upgrades>,
	energy_regen: WriteStorage<'a, EnergyRegen>,
	health_regen: WriteStorage<'a, HealthRegen>,
}

impl EventHandlerTypeProvider for InitState {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for InitState {
	type SystemData = InitStateData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		let ref info = data.config.planes[*data.plane.get(evt.id).unwrap()];
		let energy_regen = info.energy_regen;
		let health_regen = info.health_regen;

		data.energy.insert(evt.id, Energy::new(1.0)).unwrap();
		data.health.insert(evt.id, Health::new(1.0)).unwrap();
		data.keystate.insert(evt.id, KeyState::default()).unwrap();
		data.upgrades.insert(evt.id, Upgrades::default()).unwrap();
		data.energy_regen.insert(evt.id, energy_regen).unwrap();
		data.health_regen.insert(evt.id, health_regen).unwrap();
	}
}

impl SystemInfo for InitState {
	type Dependencies = (InitTraits);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

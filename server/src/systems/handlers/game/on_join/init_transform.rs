use specs::*;
use types::*;

use GameMode;
use GameModeWriter;
use SystemInfo;

use component::event::*;
use utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct InitTransform;

#[derive(SystemData)]
pub struct InitTransformData<'a> {
	gamemode: GameModeWriter<'a, dyn GameMode>,

	pos: WriteStorage<'a, Position>,
	rot: WriteStorage<'a, Rotation>,
	vel: WriteStorage<'a, Velocity>,
}

impl EventHandlerTypeProvider for InitTransform {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for InitTransform {
	type SystemData = InitTransformData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		let player_pos = data.gamemode.get_mut().spawn_pos(evt.id, evt.team);

		data.pos.insert(evt.id, player_pos).unwrap();
		data.rot.insert(evt.id, Rotation::default()).unwrap();
		data.vel.insert(evt.id, Velocity::default()).unwrap();
	}
}

impl SystemInfo for InitTransform {
	type Dependencies = (super::InitTraits);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

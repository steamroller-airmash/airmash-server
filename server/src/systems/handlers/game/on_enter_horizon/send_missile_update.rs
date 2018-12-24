use specs::*;

use component::event::*;
use protocol::server::MobUpdate;
use types::systemdata::ReadClock;
use types::systemdata::*;
use types::*;
use utils::*;
use SystemInfo;

#[derive(Default)]
pub struct SendMissileUpdate;

#[derive(SystemData)]
pub struct SendMissileUpdateData<'a> {
	conns: SendToPlayer<'a>,
	config: Read<'a, Config>,
	clock: ReadClock<'a>,

	pos: ReadStorage<'a, Position>,
	vel: ReadStorage<'a, Velocity>,
	mob: ReadStorage<'a, Mob>,
}

impl EventHandlerTypeProvider for SendMissileUpdate {
	type Event = EnterHorizon;
}

impl<'a> EventHandler<'a> for SendMissileUpdate {
	type SystemData = SendMissileUpdateData<'a>;

	fn on_event(&mut self, evt: &EnterHorizon, data: &mut Self::SystemData) {
		if evt.entered_ty != EntityType::Missile {
			return;
		}

		let pos = *try_get!(evt.entered, data.pos);
		let vel = *try_get!(evt.entered, data.vel);
		let mob = *try_get!(evt.entered, data.mob);

		let ref info = data.config.mobs[mob].missile.unwrap();
		let max_speed = info.max_speed;
		let accel = vel.normalized() * info.accel;

		data.conns.send_to_player(
			evt.player,
			MobUpdate {
				id: evt.entered.into(),
				clock: data.clock.get(),
				ty: mob,
				pos,
				speed: vel,
				accel,
				max_speed,
			},
		);
	}
}

impl SystemInfo for SendMissileUpdate {
	type Dependencies = (super::KnownEventSources);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Default::default()
	}
}

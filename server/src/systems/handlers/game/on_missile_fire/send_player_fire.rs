use specs::*;

use crate::component::event::*;
use crate::protocol::server::{PlayerFire, PlayerFireProjectile};
use crate::types::systemdata::*;
use crate::types::*;
use crate::utils::{EventHandler, EventHandlerTypeProvider};
use crate::SystemInfo;

#[derive(Default)]
pub struct SendPlayerFire;

#[derive(SystemData)]
pub struct SendPlayerFireData<'a> {
	conns: SendToVisible<'a>,
	config: Read<'a, Config>,
	clock: ReadClock<'a>,

	mob: ReadStorage<'a, Mob>,
	pos: ReadStorage<'a, Position>,
	vel: ReadStorage<'a, Velocity>,
	energy: ReadStorage<'a, Energy>,
	energy_regen: ReadStorage<'a, EnergyRegen>,
}

impl EventHandlerTypeProvider for SendPlayerFire {
	type Event = MissileFire;
}

impl<'a> EventHandler<'a> for SendPlayerFire {
	type SystemData = SendPlayerFireData<'a>;

	fn on_event(&mut self, evt: &MissileFire, data: &mut Self::SystemData) {
		let projectiles = evt
			.missiles
			.iter()
			.filter_map(|&ent| {
				let ty = *log_none!(ent, data.mob)?;
				let info = data.config.mobs[ty].missile.unwrap();

				let vel = *log_none!(ent, data.vel)?;
				let pos = *log_none!(ent, data.pos)?;

				PlayerFireProjectile {
					id: ent.into(),
					pos: pos,
					speed: vel,
					ty: ty,
					accel: vel.normalized() * info.accel,
					max_speed: info.max_speed,
				}
				.into()
			})
			.collect::<Vec<_>>();

		let pos = *try_get!(evt.player, data.pos);

		let packet = PlayerFire {
			clock: data.clock.get(),
			id: evt.player.into(),
			energy: *try_get!(evt.player, data.energy),
			energy_regen: *try_get!(evt.player, data.energy_regen),
			projectiles,
		};

		data.conns.send_to_visible(pos, packet);
	}
}

impl SystemInfo for SendPlayerFire {
	type Dependencies = super::KnownEventSources;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

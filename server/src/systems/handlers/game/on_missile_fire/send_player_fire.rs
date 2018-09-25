use specs::*;

use types::systemdata::*;
use types::*;

use dispatch::SystemInfo;
use systems::missile::MissileFireHandler;

use component::channel::*;

use airmash_protocol::server::{PlayerFire, PlayerFireProjectile};

pub struct SendPlayerFire {
	reader: Option<OnMissileFireReader>,
}

#[derive(SystemData)]
pub struct SendPlayerFireData<'a> {
	pub entities: Entities<'a>,
	pub channel: Read<'a, OnMissileFire>,
	pub conns: Read<'a, Connections>,
	pub config: Read<'a, Config>,
	pub clock: ReadClock<'a>,

	pub mob: ReadStorage<'a, Mob>,
	pub pos: ReadStorage<'a, Position>,
	pub vel: ReadStorage<'a, Velocity>,
	pub energy: ReadStorage<'a, Energy>,
	pub energy_regen: ReadStorage<'a, EnergyRegen>,
}

impl SendPlayerFire {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for SendPlayerFire {
	type SystemData = SendPlayerFireData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnMissileFire>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let projectiles = evt
				.missiles
				.iter()
				.map(|&ent| {
					let ty = *data.mob.get(ent).unwrap();
					let info = data.config.mobs[ty].missile.unwrap();

					let vel = *data.vel.get(ent).unwrap();
					let pos = *data.pos.get(ent).unwrap();

					PlayerFireProjectile {
						id: ent.into(),
						pos: pos,
						speed: vel,
						ty: ty,
						accel: vel.normalized() * info.accel,
						max_speed: info.max_speed,
					}
				}).collect::<Vec<_>>();

			let packet = PlayerFire {
				clock: data.clock.get(),
				id: evt.player.into(),
				energy: *data.energy.get(evt.player).unwrap(),
				energy_regen: *data.energy_regen.get(evt.player).unwrap(),
				projectiles,
			};

			data.conns.send_to_visible(evt.player, packet);
		}
	}
}

impl SystemInfo for SendPlayerFire {
	type Dependencies = MissileFireHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

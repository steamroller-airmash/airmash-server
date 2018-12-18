use specs::*;
use types::systemdata::*;
use types::*;

use component::channel::*;
use component::flag::*;
use component::reference::PlayerRef;

use protocol::server::{PlayerHit, PlayerHitPlayer};

pub struct SendPacket {
	reader: Option<OnPlayerHitReader>,
}

#[derive(SystemData)]
pub struct SendPacketData<'a> {
	channel: Read<'a, OnPlayerHit>,
	config: Read<'a, Config>,
	conns: SendToVisible<'a>,

	health: ReadStorage<'a, Health>,
	plane: ReadStorage<'a, Plane>,
	owner: ReadStorage<'a, PlayerRef>,

	mob: ReadStorage<'a, Mob>,
	pos: ReadStorage<'a, Position>,
	is_missile: ReadStorage<'a, IsMissile>,
}

impl SendPacket {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for SendPacket {
	type SystemData = SendPacketData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerHit>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if !data.is_missile.get(evt.missile).is_some() {
				continue;
			}

			let pos = try_get!(evt.missile, data.pos);
			let mob = try_get!(evt.missile, data.mob);
			let owner = try_get!(evt.missile, data.owner);

			let health = try_get!(evt.player, data.health);
			let plane = try_get!(evt.player, data.plane);

			let ref planeconf = data.config.planes[*plane];

			let packet = PlayerHit {
				id: evt.missile.into(),
				owner: owner.0.into(),
				pos: *pos,
				ty: *mob,
				players: vec![PlayerHitPlayer {
					id: evt.player.into(),
					health: *health,
					health_regen: planeconf.health_regen,
				}],
			};

			data.conns.send_to_visible(*pos, packet);
		}
	}
}

use super::*;
use dispatch::SystemInfo;

impl SystemInfo for SendPacket {
	type Dependencies = InflictDamage;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

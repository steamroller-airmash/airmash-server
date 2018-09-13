use specs::*;
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
	pub channel: Read<'a, OnPlayerHit>,
	pub config: Read<'a, Config>,
	pub conns: Read<'a, Connections>,

	pub health: ReadStorage<'a, Health>,
	pub plane: ReadStorage<'a, Plane>,
	pub upgrades: ReadStorage<'a, Upgrades>,
	pub owner: ReadStorage<'a, PlayerRef>,
	pub player_flag: ReadStorage<'a, IsPlayer>,

	pub mob: ReadStorage<'a, Mob>,
	pub pos: ReadStorage<'a, Position>,
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
			let pos = data.pos.get(evt.missile).unwrap();
			let mob = data.mob.get(evt.missile).unwrap();
			let owner = data.owner.get(evt.missile).unwrap();

			let health = data.health.get(evt.player).unwrap();
			let plane = data.plane.get(evt.player).unwrap();

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

			data.conns.send_to_all(packet);
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

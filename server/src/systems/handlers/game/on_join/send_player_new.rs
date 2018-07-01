
use specs::*;
use types::*;

use SystemInfo;
use OwnedMessage;

use component::channel::*;
use protocol::server::PlayerNew;
use protocol::{ServerPacket, to_bytes, Upgrades as ProtocolUpgrades};

pub struct SendPlayerNew {
	reader: Option<OnPlayerJoinReader>
}

#[derive(SystemData)]
pub struct SendPlayerNewData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,
	pub conns: Read<'a, Connections>,

	pub pos: ReadStorage<'a, Position>,
	pub rot: ReadStorage<'a, Rotation>,
	pub plane: ReadStorage<'a, Plane>,
	pub team: ReadStorage<'a, Team>,
	pub status: ReadStorage<'a, Status>,
	pub flag: ReadStorage<'a, Flag>,
	pub upgrades: ReadStorage<'a, Upgrades>,
	pub powerups: ReadStorage<'a, Powerups>,
	pub name: ReadStorage<'a, Name>,
}

impl<'a> System<'a> for SendPlayerNew {
	type SystemData = SendPlayerNewData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(
			res.fetch_mut::<OnPlayerJoin>().register_reader()
		);
	}

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			channel,
			conns,

			pos,
			rot,
			team,
			name,
			plane,
			status,
			flag,
			upgrades,
			powerups,
		} = data;

		for evt in channel.read(self.reader.as_mut().unwrap()) {
			let powerups = *powerups.get(evt.0).unwrap();

			let upgrades = ProtocolUpgrades {
				speed: upgrades.get(evt.0).unwrap().speed,
				inferno: powerups.inferno,
				shield: powerups.shield,
			};

			let player_new = PlayerNew {
				id: evt.0,
				status: *status.get(evt.0).unwrap(),
				name: name.get(evt.0).unwrap().0.clone(),
				ty: *plane.get(evt.0).unwrap(),
				team: *team.get(evt.0).unwrap(),
				pos: *pos.get(evt.0).unwrap(),
				rot: *rot.get(evt.0).unwrap(),
				flag: *flag.get(evt.0).unwrap(),
				upgrades
			};

			conns.send_to_others(evt.0, OwnedMessage::Binary(
				to_bytes(&ServerPacket::PlayerNew(player_new)).unwrap()
			));
		}
	}
}

impl SystemInfo for SendPlayerNew {
	type Dependencies = super::InitTraits;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self{ reader: None }
	}
}


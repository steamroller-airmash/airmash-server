use specs::*;
use types::*;

use SystemInfo;

use component::channel::*;
use protocol::server::PlayerNew;
use protocol::Upgrades as ProtocolUpgrades;

pub struct SendPlayerNew {
	reader: Option<OnPlayerJoinReader>,
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
	pub flag: ReadStorage<'a, FlagCode>,
	pub upgrades: ReadStorage<'a, Upgrades>,
	pub powerups: ReadStorage<'a, Powerups>,
	pub name: ReadStorage<'a, Name>,
}

impl<'a> System<'a> for SendPlayerNew {
	type SystemData = SendPlayerNewData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
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
			let powerups = *powerups.get(evt.id).unwrap();

			let upgrades = ProtocolUpgrades {
				speed: upgrades.get(evt.id).unwrap().speed,
				inferno: powerups.inferno(),
				shield: powerups.shield(),
			};

			let player_new = PlayerNew {
				id: evt.id.into(),
				status: *status.get(evt.id).unwrap(),
				name: name.get(evt.id).unwrap().0.clone(),
				ty: *plane.get(evt.id).unwrap(),
				team: *team.get(evt.id).unwrap(),
				pos: *pos.get(evt.id).unwrap(),
				rot: *rot.get(evt.id).unwrap(),
				flag: *flag.get(evt.id).unwrap(),
				upgrades,
			};

			conns.send_to_others(evt.id, player_new);
		}
	}
}

impl SystemInfo for SendPlayerNew {
	type Dependencies = (
		super::InitTraits,
		super::InitConnection,
		super::InitState,
		super::InitTransform,
	);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

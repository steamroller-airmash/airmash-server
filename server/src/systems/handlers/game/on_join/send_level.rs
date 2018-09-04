use specs::*;
use types::*;

use SystemInfo;

use component::channel::*;
use protocol::server::PlayerLevel;
use protocol::PlayerLevelType;

pub struct SendPlayerLevel {
	reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct SendPlayerLevelData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,
	pub conns: Read<'a, Connections>,

	pub level: ReadStorage<'a, Level>,
}

impl<'a> System<'a> for SendPlayerLevel {
	type SystemData = SendPlayerLevelData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			channel,
			conns,

			level,
		} = data;

		for evt in channel.read(self.reader.as_mut().unwrap()) {
			let packet = PlayerLevel {
				id: evt.id.into(),
				ty: PlayerLevelType::Login,
				level: *level.get(evt.id).unwrap(),
			};

			conns.send_to_others(evt.id, packet);
		}
	}
}

impl SystemInfo for SendPlayerLevel {
	type Dependencies = (super::InitTraits, super::SendLogin, super::InitConnection);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

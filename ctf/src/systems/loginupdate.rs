use specs::*;

use server::component::channel::*;
use server::protocol::server::{GameFlag, ServerPacket};
use server::protocol::{to_bytes, FlagUpdateType};
use server::*;

use component::*;

pub struct LoginUpdateSystem {
	reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct LoginUpdateSystemData<'a> {
	pub conns: Read<'a, Connections>,
	pub join_channel: Read<'a, OnPlayerJoin>,

	// These ones are for both
	pub pos: ReadStorage<'a, Position>,
	pub team: ReadStorage<'a, Team>,

	// Flag Data
	pub is_flag: ReadStorage<'a, IsFlag>,
	pub carrier: ReadStorage<'a, FlagCarrier>,
}

impl LoginUpdateSystem {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for LoginUpdateSystem {
	type SystemData = LoginUpdateSystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.join_channel.read(self.reader.as_mut().unwrap()) {
			(&data.pos, &data.team, &data.carrier, &data.is_flag)
				.join()
				.for_each(|(pos, team, carrier, _)| {
					let ty = match carrier.0 {
						Some(_) => FlagUpdateType::Carrier,
						None => FlagUpdateType::Position,
					};

					let packet = GameFlag {
						ty,
						flag: *team,
						pos: *pos,
						id: carrier.0,
						blueteam: 0,
						redteam: 0,
					};

					data.conns.send_to_player(
						evt.0,
						OwnedMessage::Binary(to_bytes(&ServerPacket::GameFlag(packet)).unwrap()),
					);
				});
		}
	}
}

impl SystemInfo for LoginUpdateSystem {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

use specs::*;

use server::component::channel::*;
use server::protocol::server::GameFlag;
use server::protocol::FlagUpdateType;
use server::systems::handlers::game::on_join::SendLogin;
use server::*;

use component::*;

pub struct SendFlagPosition {
	reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct SendFlagPositionData<'a> {
	pub conns: Read<'a, Connections>,
	pub join_channel: Read<'a, OnPlayerJoin>,
	pub scores: Read<'a, GameScores>,

	// These ones are for both
	pub pos: ReadStorage<'a, Position>,
	pub team: ReadStorage<'a, Team>,

	// Flag Data
	pub is_flag: ReadStorage<'a, IsFlag>,
	pub carrier: ReadStorage<'a, FlagCarrier>,
}

impl SendFlagPosition {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for SendFlagPosition {
	type SystemData = SendFlagPositionData<'a>;

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
						flag: Flag(*team),
						pos: *pos,
						id: carrier.0.map(Into::into),
						blueteam: data.scores.blueteam,
						redteam: data.scores.redteam,
					};

					data.conns.send_to_player(evt.id, packet);
				});
		}
	}
}

impl SystemInfo for SendFlagPosition {
	// The client ignores packets that are
	// sent before the login packet
	type Dependencies = SendLogin;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

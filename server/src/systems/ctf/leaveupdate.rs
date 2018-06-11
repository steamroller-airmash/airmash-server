
use specs::*;
use types::*;

use component::channel::*;
use component::ctf::*;

use websocket::OwnedMessage;
use protocol::server::GameFlag;
use protocol::{FlagUpdateType, ServerPacket, to_bytes};

pub struct LeaveUpdateSystem {
	reader: Option<OnPlayerLeaveReader>
}

#[derive(SystemData)]
pub struct LeaveUpdateSystemData<'a> {
	pub channel: Read<'a, OnPlayerLeave>,
	pub conns:   Read<'a, Connections>,
	pub pos:     WriteStorage<'a, Position>,
	pub is_flag: ReadStorage<'a, IsFlag>,
	pub carrier: WriteStorage<'a, FlagCarrier>,
	pub teams:   ReadStorage<'a, Team>,
}

impl LeaveUpdateSystem {
	pub fn new() -> Self {
		 Self { reader: None }
	}
}

impl<'a> System<'a> for LeaveUpdateSystem {
	type SystemData = LeaveUpdateSystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(
			res.fetch_mut::<OnPlayerLeave>().register_reader()
		);
	}

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			channel,
			conns,
			mut pos,
			is_flag,
			teams,
			mut carrier
		} = data;

		for evt in channel.read(self.reader.as_mut().unwrap()) {
			let player_pos = *pos.get(evt.0).unwrap();

			(&mut pos, &mut carrier, &is_flag).join()
				.filter(|(_, carrier, _)| {
					carrier.0.is_some() && carrier.0.unwrap() == evt.0
				})
				.for_each(|(pos, carrier, _)| {
					let team = teams.get(evt.0).unwrap();

					let packet = GameFlag {
						ty: FlagUpdateType::Position,
						flag: *team,
						id: None,
						pos: player_pos,
						blueteam: 0,
						redteam: 0
					};

					*pos = player_pos;
					*carrier = FlagCarrier(None);

					conns.send_to_all(OwnedMessage::Binary(
						to_bytes(&ServerPacket::GameFlag(packet)).unwrap()
					));
				});
		}
	}
}

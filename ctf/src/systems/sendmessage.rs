use server::*;
use specs::*;

use component::*;
use server::protocol::server::GameFlag;
use server::protocol::FlagUpdateType;
use server::protocol::{to_bytes, ServerPacket};

use RED_TEAM;
use BLUE_TEAM;

pub struct SendFlagMessageSystem {
	reader: Option<OnFlagReader>,
}

impl SendFlagMessageSystem {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

#[derive(SystemData)]
pub struct SendFlagMessageSystemData<'a> {
	pub conns: Read<'a, Connections>,
	pub channel: Read<'a, OnFlag>,
	pub scores: Write<'a, GameScores>,
	pub flags: ReadExpect<'a, Flags>,

	pub team: ReadStorage<'a, Team>,
	pub pos: ReadStorage<'a, Position>,
	pub carrier: ReadStorage<'a, FlagCarrier>,
}

impl<'a> System<'a> for SendFlagMessageSystem {
	type SystemData = SendFlagMessageSystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnFlag>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let ty = match evt.ty {
				FlagEventType::PickUp => FlagUpdateType::Carrier,
				_ => FlagUpdateType::Position,
			};

			let team = data.team.get(evt.flag).unwrap();

			if evt.ty == FlagEventType::Capture {
				let other;
				if *team == RED_TEAM {
					data.scores.blueteam += 1;
					other = data.flags.blue;
				}
				else if *team == BLUE_TEAM {
					data.scores.redteam += 1;
					other = data.flags.red;
				}
				else {
					unimplemented!();
				}

				let pos;			
				if data.carrier.get(other).unwrap().0.is_none() {
					pos = *data.pos.get(other).unwrap();
				}
				else {
					pos = Position::default();
				}

				data.conns.send_to_all(OwnedMessage::Binary(
					to_bytes(&ServerPacket::GameFlag(GameFlag {
						ty,
						flag: *data.team.get(other).unwrap(),
						pos: pos,
						id: None,
						blueteam: data.scores.blueteam,
						redteam: data.scores.redteam,
					})).unwrap()
				));

				info!("{:?}", *data.scores);
			}

			data.conns.send_to_all(OwnedMessage::Binary(
				to_bytes(&ServerPacket::GameFlag(GameFlag {
					ty,
					flag: *team,
					pos: *data.pos.get(evt.flag).unwrap(),
					id: evt.player,
					blueteam: data.scores.blueteam,
					redteam: data.scores.redteam,
				})).unwrap(),
			));

			
		}
	}
}

use super::PickupFlagSystem;
use std::any::Any;

impl SystemInfo for SendFlagMessageSystem {
	type Dependencies = PickupFlagSystem;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new(_: Box<Any>) -> Self {
		Self::new()
	}
}

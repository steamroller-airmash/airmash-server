use server::*;
use specs::*;

use component::*;
use server::protocol::server::GameFlag;
use server::protocol::FlagUpdateType;
use server::protocol::{to_bytes, ServerPacket};

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

	pub team: ReadStorage<'a, Team>,
	pub pos: ReadStorage<'a, Position>,
}

impl<'a> System<'a> for SendFlagMessageSystem {
	type SystemData = SendFlagMessageSystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnFlag>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let ty = match evt.ty {
				FlagEventType::PickUp => FlagUpdateType::Carrier,
				_ => FlagUpdateType::Position,
			};

			let team = data.team.get(evt.flag).unwrap();
			let pos = data.pos.get(evt.flag).unwrap();

			let packet = GameFlag {
				ty,
				flag: *team,
				pos: *pos,
				id: evt.player,
				blueteam: 0,
				redteam: 0,
			};

			data.conns.send_to_all(OwnedMessage::Binary(
				to_bytes(&ServerPacket::GameFlag(packet)).unwrap(),
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

use shrev::*;
use specs::*;

use std::any::Any;

use types::*;

use consts::timer::SCORE_BOARD;
use dispatch::SystemInfo;
use systems;

use component::channel::*;
use component::event::TimerEvent;
use component::time::ThisFrame;

use protocol::server::PlayerKill;
use protocol::{to_bytes, ServerPacket};
use websocket::OwnedMessage;

pub struct DisplayMessage {
	reader: Option<OnPlayerKilledReader>,
}

#[derive(SystemData)]
pub struct DisplayMessageData<'a> {
	pub entities: Entities<'a>,
	pub channel: Read<'a, OnPlayerKilled>,
	pub conns: Read<'a, Connections>,
	pub timerevent: Write<'a, EventChannel<TimerEvent>>,
	pub thisframe: Read<'a, ThisFrame>,
}

impl DisplayMessage {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for DisplayMessage {
	type SystemData = DisplayMessageData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerKilled>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let packet = PlayerKill {
				id: evt.player,
				killer: Some(evt.killer),
				pos: evt.pos,
			};

			data.conns.send_to_all(OwnedMessage::Binary(
				to_bytes(&ServerPacket::PlayerKill(packet)).unwrap(),
			));

			data.timerevent.single_write(TimerEvent {
				ty: *SCORE_BOARD,
				instant: data.thisframe.0,
				..Default::default()
			});
		}
	}
}

impl SystemInfo for DisplayMessage {
	type Dependencies = (systems::missile::MissileHit);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new(_: Box<Any>) -> Self {
		Self::new()
	}
}

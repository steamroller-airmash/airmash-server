
use specs::*;
use shrev::*;

use std::any::Any;

use types::*;

use systems;
use consts::timer::SCORE_BOARD;
use dispatch::SystemInfo;

use component::channel::*;
use component::time::ThisFrame;
use component::event::TimerEvent;

use websocket::OwnedMessage;
use protocol::{to_bytes, ServerPacket};
use protocol::server::PlayerKill;

pub struct PlayerKilledMessage {
	reader: Option<OnPlayerKilledReader>
}

#[derive(SystemData)]
pub struct PlayerKilledMessageData<'a> {
	pub entities: Entities<'a>,
	pub channel: Read<'a, OnPlayerKilled>,
	pub conns: Read<'a, Connections>,
	pub timerevent: Write<'a, EventChannel<TimerEvent>>,
	pub thisframe: Read<'a, ThisFrame>,

	pub name: ReadStorage<'a, Name>,
	pub level: ReadStorage<'a, Level>,
}

impl PlayerKilledMessage {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for PlayerKilledMessage {
	type SystemData = PlayerKilledMessageData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(
			res.fetch_mut::<OnPlayerKilled>().register_reader()
		);
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let packet = PlayerKill {
				id: evt.player,
				killer: Some(evt.killer),
				pos: evt.pos
			};

			data.conns.send_to_all(OwnedMessage::Binary(
				to_bytes(&ServerPacket::PlayerKill(packet)).unwrap()
			));

			data.timerevent.single_write(TimerEvent {
				ty: *SCORE_BOARD,
				instant: data.thisframe.0
			});
		}
	}
}

impl SystemInfo for PlayerKilledMessage {
	type Dependencies = (
		systems::missile::MissileHit
	);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new(_: Box<Any>) -> Self {
		Self::new()
	}
}


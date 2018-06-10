use shrev::*;
use specs::*;
use types::*;

use airmash_protocol::client::Say;
use airmash_protocol::server::{ChatSay, ServerPacket};
use airmash_protocol::to_bytes;
use websocket::OwnedMessage;

pub struct SayHandler {
	reader: Option<ReaderId<(ConnectionId, Say)>>,
}

#[derive(SystemData)]
pub struct SayHandlerData<'a> {
	channel: Read<'a, EventChannel<(ConnectionId, Say)>>,
	conns: Read<'a, Connections>,
}

impl SayHandler {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for SayHandler {
	type SystemData = SayHandlerData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		self.reader = Some(
			res.fetch_mut::<EventChannel<(ConnectionId, Say)>>()
				.register_reader(),
		);

		Self::SystemData::setup(res);
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let player = match data.conns.0.get(&evt.0) {
				Some(data) => match data.player {
					Some(player) => player,
					None => continue,
				},
				None => continue,
			};

			let chat = ChatSay {
				id: player.id() as u16,
				text: evt.1.text.clone(),
			};

			data.conns.send_to_all(OwnedMessage::Binary(
				to_bytes(&ServerPacket::ChatSay(chat)).unwrap(),
			));
		}
	}
}

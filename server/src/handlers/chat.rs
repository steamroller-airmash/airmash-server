use shrev::*;
use specs::*;
use types::*;

use airmash_protocol::client::Chat;
use airmash_protocol::server::{ChatPublic, ServerPacket};
use airmash_protocol::to_bytes;
use websocket::OwnedMessage;

pub struct ChatHandler {
	reader: Option<ReaderId<(ConnectionId, Chat)>>,
}

#[derive(SystemData)]
pub struct ChatHandlerData<'a> {
	channel: Read<'a, EventChannel<(ConnectionId, Chat)>>,
	conns: Read<'a, Connections>,
}

impl ChatHandler {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for ChatHandler {
	type SystemData = ChatHandlerData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		self.reader = Some(
			res.fetch_mut::<EventChannel<(ConnectionId, Chat)>>()
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

			let chat = ChatPublic {
				id: player,
				text: evt.1.text.clone(),
			};

			data.conns.send_to_all(OwnedMessage::Binary(
				to_bytes(&ServerPacket::ChatPublic(chat)).unwrap(),
			));
		}
	}
}

use dispatch::SystemInfo;
use handlers::OnCloseHandler;

impl SystemInfo for ChatHandler {
	type Dependencies = OnCloseHandler;

	fn new() -> Self {
		Self::new()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

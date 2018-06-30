
use shrev::*;
use specs::*;
use types::*;

use protocol::client::Whisper;
use protocol::server::{ChatWhisper, ServerPacket};
use protocol::to_bytes;
use OwnedMessage;

use component::flag::IsPlayer;

pub struct WhisperHandler {
	reader: Option<ReaderId<(ConnectionId, Whisper)>>,
}

#[derive(SystemData)]
pub struct WhisperHandlerData<'a> {
	channel: Read<'a, EventChannel<(ConnectionId, Whisper)>>,
	conns: Read<'a, Connections>,

	entities: Entities<'a>,
	is_player: ReadStorage<'a, IsPlayer>,
}

impl WhisperHandler {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for WhisperHandler {
	type SystemData = WhisperHandlerData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(
			res.fetch_mut::<EventChannel<(ConnectionId, Whisper)>>()
				.register_reader(),
		);
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			info!("{:?}", evt);
			let player = match data.conns.0.get(&evt.0) {
				Some(data) => match data.player {
					Some(player) => player,
					None => continue,
				},
				None => continue,
			};

			let to = data.entities.entity(evt.1.id as u32);

			if !data.entities.is_alive(to) {
				// The player doesn't exist
				continue;
			}
			if data.is_player.get(to).is_none() {
				// Entity is not a player
				continue;
			}

			let chat = ChatWhisper {
				from: player,
				to: to,
				text: evt.1.text.clone(),
			};

			let packet = ServerPacket::ChatWhisper(chat);

			data.conns.send_to(evt.0, OwnedMessage::Binary(
				to_bytes(&packet).unwrap(),
			));

			data.conns.send_to_player(to, OwnedMessage::Binary(
				to_bytes(&packet).unwrap(),
			));
		}
	}
}

use dispatch::SystemInfo;
use handlers::OnCloseHandler;

impl SystemInfo for WhisperHandler {
	type Dependencies = OnCloseHandler;

	fn new() -> Self {
		Self::new()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}


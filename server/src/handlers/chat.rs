use shrev::*;
use specs::*;
use types::systemdata::*;
use types::*;

use protocol::client::Chat;
use protocol::server::{ChatPublic, Error};
use protocol::ErrorType;

use component::flag::{IsChatMuted, IsChatThrottled};

pub struct ChatHandler {
	reader: Option<ReaderId<(ConnectionId, Chat)>>,
}

#[derive(SystemData)]
pub struct ChatHandlerData<'a> {
	channel: Read<'a, EventChannel<(ConnectionId, Chat)>>,
	conns: SendToAll<'a>,

	throttled: ReadStorage<'a, IsChatThrottled>,
	muted: ReadStorage<'a, IsChatMuted>,
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
			let player = match data.conns.conns.associated_player(evt.0) {
				Some(player) => player,
				None => continue,
			};

			if data.muted.get(player).is_some() {
				continue;
			}
			if data.throttled.get(player).is_some() {
				data.conns.send_to(
					evt.0,
					Error {
						error: ErrorType::ChatThrottled,
					},
				);
				continue;
			}

			data.conns.send_to_all(ChatPublic {
				id: player.into(),
				text: evt.1.text.clone(),
			});
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

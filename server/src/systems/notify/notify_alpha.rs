use specs::*;
use types::*;

use SystemInfo;

use std::time::Duration;

use component::channel::*;
use protocol::server::ServerMessage;
use protocol::ServerMessageType;

pub struct NotifyAlpha {
	reader: Option<OnPlayerJoinReader>,
	duration: Duration,
	message: String,
}

#[derive(SystemData)]
pub struct NotifyAlphaData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,
	pub conns: Read<'a, Connections>,
}

impl<'a> System<'a> for NotifyAlpha {
	type SystemData = NotifyAlphaData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let packet = ServerMessage {
				ty: ServerMessageType::Banner,
				text: self.message.clone(),
				duration: (self.duration.as_secs() * 1000) as u32 + self.duration.subsec_millis(),
			};

			data.conns.send_to_player(evt.id, packet);
		}
	}
}

impl SystemInfo for NotifyAlpha {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self {
			reader: None,
			// Note: don't set this to a duration above approximately
			// 49.7 weeks so that the number of milliseconds does not
			// overflow a u32
			duration: Duration::from_secs(5),
			message: "This server is in alpha! Don't expect things to work correctly or at all."
				.to_string(),
		}
	}
}

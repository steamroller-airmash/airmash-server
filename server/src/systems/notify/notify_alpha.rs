use crate::SystemInfo;

use std::time::Duration;

use crate::component::event::PlayerJoin;
use crate::protocol::server::ServerMessage;
use crate::protocol::ServerMessageType;
use crate::types::systemdata::*;
use crate::utils::*;

pub struct NotifyAlpha {
	duration: Duration,
	message: String,
}

#[derive(SystemData)]
pub struct NotifyAlphaData<'a> {
	conns: SendToPlayer<'a>,
}

impl EventHandlerTypeProvider for NotifyAlpha {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for NotifyAlpha {
	type SystemData = NotifyAlphaData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		let packet = ServerMessage {
			ty: ServerMessageType::Banner,
			text: self.message.clone(),
			duration: (self.duration.as_secs() * 1000) as u32 + self.duration.subsec_millis(),
		};

		data.conns.send_to_player(evt.id, packet);
	}
}

impl SystemInfo for NotifyAlpha {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self {
			// Note: don't set this to a duration above approximately
			// 49.7 weeks so that the number of milliseconds does not
			// overflow a u32
			duration: Duration::from_secs(5),
			message: "This server is in alpha! Don't expect things to work correctly or at all."
				.to_string(),
		}
	}
}

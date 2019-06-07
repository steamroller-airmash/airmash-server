use specs::*;

use component::event::SayEvent;
use component::flag::*;
use types::systemdata::*;
use types::*;

use protocol::server::{ChatSay, Error};
use protocol::ErrorType;
use utils::*;

#[derive(Default)]
pub struct SayHandler;

#[derive(SystemData)]
pub struct SayHandlerData<'a> {
	conns: SendToVisible<'a>,

	throttled: ReadStorage<'a, IsChatThrottled>,
	muted: ReadStorage<'a, IsChatMuted>,
	pos: ReadStorage<'a, Position>,
}

impl EventHandlerTypeProvider for SayHandler {
	type Event = SayEvent;
}

impl<'a> EventHandler<'a> for SayHandler {
	type SystemData = SayHandlerData<'a>;

	fn on_event(&mut self, evt: &SayEvent, data: &mut Self::SystemData) {
		let player = match data.conns.associated_player(evt.0) {
			Some(player) => player,
			None => return,
		};

		if data.muted.get(player).is_some() {
			return;
		}
		if data.throttled.get(player).is_some() {
			data.conns.send_to(
				evt.0,
				Error {
					error: ErrorType::ChatThrottled,
				},
			);
			return;
		}

		let chat = ChatSay {
			id: player.into(),
			text: evt.1.text.clone(),
		};

		let pos = *try_get!(player, data.pos);
		info!("{:?} {:?}", evt, pos);

		data.conns.send_to_visible(pos, chat);
	}
}

use dispatch::SystemInfo;
use handlers::OnCloseHandler;

impl SystemInfo for SayHandler {
	type Dependencies = OnCloseHandler;

	fn new() -> Self {
		Self::default()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

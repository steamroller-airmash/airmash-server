use specs::*;

use types::systemdata::*;
use types::*;

use protocol::client::TeamChat;
use protocol::server::{ChatTeam, Error};
use protocol::ErrorType;

use component::flag::{IsChatMuted, IsChatThrottled};

use utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct TeamChatHandler;

#[derive(SystemData)]
pub struct TeamChatHandlerData<'a> {
	conns: SendToTeam<'a>,

	throttled: ReadStorage<'a, IsChatThrottled>,
	muted: ReadStorage<'a, IsChatMuted>,
	team: ReadStorage<'a, Team>,
}

impl EventHandlerTypeProvider for TeamChatHandler {
	type Event = (ConnectionId, TeamChat);
}

impl<'a> EventHandler<'a> for TeamChatHandler {
	type SystemData = TeamChatHandlerData<'a>;

	fn on_event(&mut self, evt: &(ConnectionId, TeamChat), data: &mut Self::SystemData) {
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

		let team = *try_get!(player, data.team);

		data.conns.send_to_team(
			team,
			ChatTeam {
				id: player.into(),
				text: evt.1.text.clone(),
			},
		);
	}
}

use dispatch::SystemInfo;
use handlers::OnCloseHandler;

impl SystemInfo for TeamChatHandler {
	type Dependencies = OnCloseHandler;

	fn new() -> Self {
		Self::default()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

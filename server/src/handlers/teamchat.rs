use specs::prelude::*;

use crate::component::flag::{IsChatMuted, IsChatThrottled};
use crate::protocol::client::TeamChat;
use crate::protocol::server::{ChatTeam, Error};
use crate::protocol::ErrorType;
use crate::types::systemdata::Connections;
use crate::types::*;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct TeamChatHandler;

#[derive(SystemData, EventDeps)]
pub struct TeamChatHandlerData<'a> {
	conns: Connections<'a>,

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

system_info! {
	impl SystemInfo for TeamChatHandler {
		type Dependencies = super::OnCloseHandler;
	}
}

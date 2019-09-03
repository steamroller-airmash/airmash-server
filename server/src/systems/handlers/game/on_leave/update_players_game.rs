use crate::component::event::PlayerLeave;
use crate::consts::NUM_PLAYERS;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

use std::sync::atomic::Ordering::Relaxed;

#[derive(Default)]
pub struct UpdatePlayersGame;

impl EventHandlerTypeProvider for UpdatePlayersGame {
	type Event = PlayerLeave;
}

impl<'a> EventHandler<'a> for UpdatePlayersGame {
	type SystemData = ();

	fn on_event(&mut self, _: &PlayerLeave, _: &mut ()) {
		NUM_PLAYERS.fetch_sub(1, Relaxed);
	}
}

system_info! {
	impl SystemInfo for UpdatePlayersGame {
		type Dependencies = super::KnownEventSources;
	}
}

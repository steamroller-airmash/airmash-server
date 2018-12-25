use SystemInfo;

use component::event::PlayerPowerup;
use protocol::server::PlayerPowerup as ServerPlayerPowerup;
use protocol::PowerupType;

use types::systemdata::*;
use utils::{EventHandler, EventHandlerTypeProvider};

/// Add the initial 2s shield when a player joins
/// and send that packet to all visible players.
#[derive(Default)]
pub struct SendPlayerPowerup;

#[derive(SystemData)]
pub struct SendPlayerPowerupData<'a> {
	conns: SendToPlayer<'a>,
}

impl EventHandlerTypeProvider for SendPlayerPowerup {
	type Event = PlayerPowerup;
}

impl<'a> EventHandler<'a> for SendPlayerPowerup {
	type SystemData = SendPlayerPowerupData<'a>;

	fn on_event(&mut self, evt: &PlayerPowerup, data: &mut Self::SystemData) {
		let duration = evt.duration.as_secs() * 1000 + evt.duration.subsec_millis() as u64;

		data.conns.send_to_player(
			evt.player,
			ServerPlayerPowerup {
				duration: duration as u32,
				ty: PowerupType::Shield,
			},
		);
	}
}

impl SystemInfo for SendPlayerPowerup {
	type Dependencies = (super::KnownEventSources);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

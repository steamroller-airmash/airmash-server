use specs::*;

use component::channel::*;
use component::event::*;

use protocol::PowerupType;

use types::Config;
use utils::{EventHandler, EventHandlerTypeProvider};
use SystemInfo;

use super::KnownEventSources;

/// When a player respawns, give them a
/// shield for 2 seconds.
#[derive(Default)]
pub struct GiveShield;

#[derive(SystemData)]
pub struct GiveShieldData<'a> {
	channel: Write<'a, OnPlayerPowerup>,
	config: Read<'a, Config>,
}

impl EventHandlerTypeProvider for GiveShield {
	type Event = PlayerRespawn;
}

impl<'a> EventHandler<'a> for GiveShield {
	type SystemData = GiveShieldData<'a>;

	fn on_event(&mut self, evt: &PlayerRespawn, data: &mut Self::SystemData) {
		data.channel.single_write(PlayerPowerup {
			player: evt.player,
			duration: data.config.spawn_shield_duration,
			ty: PowerupType::Shield,
		});
	}
}

impl SystemInfo for GiveShield {
	type Dependencies = KnownEventSources;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

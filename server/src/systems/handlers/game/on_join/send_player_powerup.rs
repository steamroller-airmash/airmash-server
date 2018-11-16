use specs::*;

use SystemInfo;

use component::channel::OnPlayerPowerup;
use component::event::PlayerJoin;
use component::event::PlayerPowerup;

use protocol::PowerupType;

use types::Config;
use utils::event_handler::{EventHandler, EventHandlerTypeProvider};

/// Give the newly joined player an initial 2s shield.
#[derive(Default)]
pub struct SendPlayerPowerup;

#[derive(SystemData)]
pub struct SendPlayerPowerupData<'a> {
	config: Read<'a, Config>,
	channel: Write<'a, OnPlayerPowerup>,
}

impl EventHandlerTypeProvider for SendPlayerPowerup {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for SendPlayerPowerup {
	type SystemData = SendPlayerPowerupData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		data.channel.single_write(PlayerPowerup {
			player: evt.id,
			duration: data.config.spawn_shield_duration,
			ty: PowerupType::Shield,
		});
	}
}

impl SystemInfo for SendPlayerPowerup {
	type Dependencies = (super::InitState, super::SendLogin);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

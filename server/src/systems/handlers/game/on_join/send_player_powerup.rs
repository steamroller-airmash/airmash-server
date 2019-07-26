use specs::*;

use crate::SystemInfo;

use crate::component::channel::OnPlayerPowerup;
use crate::component::event::PlayerJoin;
use crate::component::event::PlayerPowerup;

use crate::protocol::PowerupType;

use crate::types::Config;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

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
	type Dependencies = (super::SendLogin);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

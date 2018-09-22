use specs::*;

use types::{Config, Connections, PowerupDetails};
use SystemInfo;

use component::event::PlayerJoin;
use component::time::ThisFrame;

use protocol::server::PlayerPowerup;
use protocol::PowerupType;

use types::Powerups;
use utils::event_handler::{EventHandler, EventHandlerTypeProvider};

/// Add the initial 2s shield when a player joins
/// and send that packet to all visible players.
#[derive(Default)]
pub struct SendPlayerPowerup;

#[derive(SystemData)]
pub struct SendPlayerPowerupData<'a> {
	conns: Read<'a, Connections>,
	config: Read<'a, Config>,
	this_frame: Read<'a, ThisFrame>,

	powerups: WriteStorage<'a, Powerups>,
}

impl EventHandlerTypeProvider for SendPlayerPowerup {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for SendPlayerPowerup {
	type SystemData = SendPlayerPowerupData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		let powerup = data.powerups.get_mut(evt.id).unwrap();

		powerup.details = Some(PowerupDetails {
			ty: PowerupType::Shield,
			end_time: data.this_frame.0 + data.config.spawn_shield_duration,
		});

		let duration = data.config.spawn_shield_duration.as_secs() * 1000
			+ data.config.spawn_shield_duration.subsec_millis() as u64;

		data.conns.send_to_player(
			evt.id,
			PlayerPowerup {
				duration: duration as u32,
				ty: PowerupType::Shield,
			},
		);
	}
}

impl SystemInfo for SendPlayerPowerup {
	type Dependencies = (super::InitState,);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

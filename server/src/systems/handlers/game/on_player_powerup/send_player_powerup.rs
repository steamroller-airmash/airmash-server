use crate::component::event::PlayerPowerup;
use crate::protocol::server::PlayerPowerup as ServerPlayerPowerup;

use crate::types::systemdata::Connections;

/// Add the initial 2s shield when a player joins
/// and send that packet to all visible players.
#[event_handler(name=SendPlayerPowerup)]
fn send_packet<'a>(evt: &PlayerPowerup, conns: &Connections<'a>) {
	let duration = evt.duration.as_secs() * 1000 + evt.duration.subsec_millis() as u64;

	conns.send_to_player(
		evt.player,
		ServerPlayerPowerup {
			duration: duration as u32,
			ty: evt.ty,
		},
	);
}

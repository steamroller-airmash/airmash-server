use specs::prelude::*;

use crate::component::event::PlayerSpectate;
use crate::protocol::server::GameSpectate;
use crate::types::systemdata::Connections;
use crate::utils::*;

#[derive(Default)]
pub struct SendSpectatePacket;

#[derive(SystemData)]
pub struct SendSpectatePacketData<'a> {
	conns: Connections<'a>,
}

impl EventHandlerTypeProvider for SendSpectatePacket {
	type Event = PlayerSpectate;
}

impl<'a> EventHandler<'a> for SendSpectatePacket {
	type SystemData = SendSpectatePacketData<'a>;

	fn on_event(&mut self, evt: &PlayerSpectate, data: &mut Self::SystemData) {
		// GameSpectate only gets sent if there
		// is someone to spectate
		if evt.target.is_none() {
			return;
		}

		let packet = GameSpectate {
			id: evt.target.unwrap().into(),
		};

		data.conns.send_to_player(evt.player, packet);
	}
}
system_info! {
	impl SystemInfo for SendSpectatePacket {
		type Dependencies = super::KnownEventSources;
	}
}

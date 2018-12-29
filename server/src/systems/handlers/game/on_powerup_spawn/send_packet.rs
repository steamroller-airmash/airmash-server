use component::event::PowerupSpawnEvent;
use types::systemdata::SendToVisible;
use utils::*;

use protocol::server::MobUpdateStationary;

#[derive(Default)]
pub struct SendPacket;

#[derive(SystemData)]
pub struct SendPacketData<'a> {
	conns: SendToVisible<'a>,
}

impl EventHandlerTypeProvider for SendPacket {
	type Event = PowerupSpawnEvent;
}

impl<'a> EventHandler<'a> for SendPacket {
	type SystemData = SendPacketData<'a>;

	fn on_event(&mut self, evt: &PowerupSpawnEvent, data: &mut Self::SystemData) {
		data.conns.send_to_visible(
			evt.pos,
			MobUpdateStationary {
				id: evt.mob.into(),
				ty: evt.ty,
				pos: evt.pos,
			},
		);
	}
}

system_info! {
	impl SystemInfo for SendPacket {
		type Dependencies = super::KnownEventSources;
	}
}

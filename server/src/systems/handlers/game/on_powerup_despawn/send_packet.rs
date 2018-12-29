use component::event::PowerupDespawnEvent;
use protocol::server::MobDespawn;
use protocol::DespawnType;
use systems;
use types::systemdata::SendToVisible;
use utils::*;

#[derive(Default)]
pub struct SendPacket;

impl EventHandlerTypeProvider for SendPacket {
	type Event = PowerupDespawnEvent;
}

impl<'a> EventHandler<'a> for SendPacket {
	type SystemData = SendToVisible<'a>;

	fn on_event(&mut self, evt: &Self::Event, conns: &mut Self::SystemData) {
		let ty = match evt.player {
			Some(_) => DespawnType::Collided,
			None => DespawnType::LifetimeEnded,
		};

		conns.send_to_visible(
			evt.pos,
			MobDespawn {
				id: evt.mob.into(),
				ty,
			},
		);
	}
}

system_info! {
	impl SystemInfo for SendPacket {
		type Dependencies = (
			super::KnownEventSources,
			systems::collision::GenPlaneGrid,
		);
	}
}

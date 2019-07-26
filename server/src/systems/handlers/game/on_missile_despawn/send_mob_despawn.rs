use crate::component::event::{MissileDespawn, MissileDespawnType};
use crate::protocol::server::MobDespawn;
use crate::protocol::DespawnType;

use crate::types::systemdata::*;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

/// Add the initial 2s shield when a player joins
/// and send that packet to all visible players.
#[derive(Default)]
pub struct SendMobDespawn;

#[derive(SystemData)]
pub struct SendMobDespawnData<'a> {
	conns: SendToVisible<'a>,
}

impl EventHandlerTypeProvider for SendMobDespawn {
	type Event = MissileDespawn;
}

impl<'a> EventHandler<'a> for SendMobDespawn {
	type SystemData = SendMobDespawnData<'a>;

	fn on_event(&mut self, evt: &MissileDespawn, data: &mut Self::SystemData) {
		let ty = match evt.ty {
			MissileDespawnType::HitPlayer => DespawnType::Collided,
			MissileDespawnType::HitTerrain => DespawnType::Collided,
			MissileDespawnType::LifetimeEnded => DespawnType::LifetimeEnded,
		};

		data.conns.send_to_visible(
			evt.pos,
			MobDespawn {
				id: evt.missile.into(),
				ty,
			},
		);
	}
}

system_info! {
	impl SystemInfo for SendMobDespawn {
		type Dependencies = (
			super::SendMobDespawnCoords,
			super::KnownEventSources
		);
	}
}

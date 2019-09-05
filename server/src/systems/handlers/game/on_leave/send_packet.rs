use specs::prelude::*;

use crate::component::event::PlayerLeave as EvtPlayerLeave;
use crate::protocol::server::PlayerLeave;
use crate::types::systemdata::Connections;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

/// Create a despawn event when a player leaves
#[derive(Default)]
pub struct SendPlayerLeave;

#[derive(SystemDataCustom)]
pub struct SendPlayerLeaveData<'a> {
	conns: Connections<'a>,
}

impl EventHandlerTypeProvider for SendPlayerLeave {
	type Event = EvtPlayerLeave;
}

impl<'a> EventHandler<'a> for SendPlayerLeave {
	type SystemData = SendPlayerLeaveData<'a>;

	fn on_event(&mut self, evt: &EvtPlayerLeave, data: &mut Self::SystemData) {
		data.conns.send_to_all(PlayerLeave { id: evt.0.into() });
	}
}

system_info! {
	impl SystemInfo for SendPlayerLeave {
		type Dependencies = super::KnownEventSources;
	}
}

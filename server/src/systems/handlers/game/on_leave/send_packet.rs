use crate::component::event::PlayerLeave as EvtPlayerLeave;
use crate::protocol::server::PlayerLeave;
use crate::types::systemdata::*;
use crate::utils::{EventHandler, EventHandlerTypeProvider};
use crate::SystemInfo;

/// Create a despawn event when a player leaves
#[derive(Default)]
pub struct SendPlayerLeave;

#[derive(SystemData)]
pub struct SendPlayerLeaveData<'a> {
	conns: SendToAll<'a>,
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

impl SystemInfo for SendPlayerLeave {
	type Dependencies = super::KnownEventSources;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

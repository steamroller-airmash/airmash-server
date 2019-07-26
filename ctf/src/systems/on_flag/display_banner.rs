use crate::server::*;
use specs::*;

use crate::component::*;
use crate::config as ctfconfig;

use htmlescape;

use crate::server::protocol::server::ServerMessage;
use crate::server::protocol::ServerMessageType;
use crate::server::types::systemdata::*;
use crate::server::utils::*;

#[derive(Default)]
pub struct PickupMessageSystem;

#[derive(SystemData)]
pub struct PickupMessageSystemData<'a> {
	conns: SendToAll<'a>,

	names: ReadStorage<'a, Name>,
	teams: ReadStorage<'a, Team>,
}

impl EventHandlerTypeProvider for PickupMessageSystem {
	type Event = FlagEvent;
}

impl<'a> EventHandler<'a> for PickupMessageSystem {
	type SystemData = PickupMessageSystemData<'a>;

	fn on_event(&mut self, evt: &FlagEvent, data: &mut Self::SystemData) {
		let verb = match evt.ty {
			FlagEventType::Return => "Returned",
			FlagEventType::PickUp => "Taken",
			FlagEventType::Capture => "Captured",
			FlagEventType::Drop => return,
		};

		// If this event happens on it's own
		// (end of game or system event) then
		// don't display a message
		if evt.player.is_none() {
			return;
		}

		let flag_team = try_get!(evt.flag, data.teams);
		let name = try_get!(evt.player.unwrap(), data.names);

		let msg = format!(
			"<span class=\"info inline\"><span class=\"{}\"></span></span>{} by {}",
			ctfconfig::FLAG_MESSAGE_TEAM[&flag_team],
			verb,
			htmlescape::encode_minimal(&name.0)
		);

		let packet = ServerMessage {
			ty: ServerMessageType::Flag,
			duration: 3000,
			text: msg,
		};

		data.conns.send_to_all(packet);
	}
}

use crate::systems::PickupFlagSystem;

impl SystemInfo for PickupMessageSystem {
	type Dependencies = PickupFlagSystem;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

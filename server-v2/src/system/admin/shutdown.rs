use crate::ecs::prelude::*;
use crate::protocol::client::Command;
use crate::resource::{
	packet::ClientPacket,
	builtin::ShutdownFlag,
	Config
};

#[event_handler]
fn shutdown<'a>(
	evt: &ClientPacket<Command>,
	
	config: &Read<'a, Config>,
	flag: &mut WriteExpect<'a, ShutdownFlag>,
) {
	if !config.admin_enabled {
		return;
	}

	if evt.packet.com != "shutdown" {
		return;
	}

	flag.shutdown();
}

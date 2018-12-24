use specs::*;
use types::*;

use std::env;

use component::event::*;
use systems::PacketHandler;
use SystemInfo;

use utils::{EventHandler, EventHandlerTypeProvider};

/// Crash the server (for testing purposes)
#[derive(Default)]
pub struct Crash;

#[derive(SystemData)]
pub struct CrashData<'a> {
	name: ReadStorage<'a, Name>,
	conns: Read<'a, Connections>,
}

impl EventHandlerTypeProvider for Crash {
	type Event = CommandEvent;
}

impl<'a> EventHandler<'a> for Crash {
	type SystemData = CrashData<'a>;

	fn on_event(&mut self, evt: &CommandEvent, data: &mut Self::SystemData) {
		let &(conn, ref packet) = evt;

		let player = match data.conns.associated_player(conn) {
			Some(p) => p,
			None => return,
		};

		if packet.com != "crash" {
			return;
		}

		let key = match env::var("AIRMASH_CRASH_KEY") {
			Ok(x) => x,
			// No key means that the crash command is disabled
			Err(_) => return,
		};

		if key != packet.data {
			return;
		}

		// Players with no names aren't allowed to crash
		// servers on purpose.
		let name = try_get!(player, data.name);

		panic!(
			"Server Debug Crash! (Triggered by {})",
			name.0
		);
	}
}

impl SystemInfo for Crash {
	type Dependencies = PacketHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

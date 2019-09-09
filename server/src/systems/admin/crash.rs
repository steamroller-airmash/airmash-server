use specs::prelude::*;

use std::env;

use crate::{
	component::event::CommandEvent,
	types::{systemdata::Connections, Config, Name},
};

/// Crash the server (for testing purposes)
#[event_handler(name=Crash)]
fn crash<'a>(
	evt: &CommandEvent,
	name: &ReadStorage<'a, Name>,
	config: &Read<'a, Config>,
	conns: &Connections<'a>,
) {
	let &(conn, ref packet) = evt;

	if !config.admin_enabled {
		return;
	}

	let player = match conns.associated_player(conn) {
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

	// Players without names aren't allowed to crash
	// servers on purpose.
	let name = try_get!(player, name);

	panic!("Server Debug Crash! (Triggered by {})", name.0);
}

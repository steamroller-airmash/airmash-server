use specs::prelude::*;

use crate::component::event::*;
use crate::protocol::server::PlayerFlag;
use crate::protocol::FlagCode;
use crate::types::systemdata::Connections;

use std::str::FromStr;

/// Handles the `/flag` command.
///
/// This system handles the `/flag` command, and sends
/// out the the `PlayerFlag` packet when it does so.
#[event_handler(name = Flag)]
fn handle_flag<'a>(
	evt: &CommandEvent,
	conns: &Connections<'a>,
	flags: &mut WriteStorage<'a, FlagCode>,
) {
	let &(conn, ref packet) = evt;

	let player = match conns.associated_player(conn) {
		Some(p) => p,
		None => return,
	};

	if packet.com != "flag" {
		return;
	}

	let flag = FlagCode::from_str(&packet.data).unwrap_or(FlagCode::UnitedNations);

	conns.send_to_all(PlayerFlag {
		id: player.into(),
		flag: flag,
	});

	flags.insert(player, flag).unwrap();
}

use specs::prelude::*;

use crate::component::event::*;
use crate::protocol::server::PlayerFlag;
use crate::protocol::FlagCode;
use crate::types::systemdata::Connections;

use crate::systems::PacketHandler;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

use std::str::FromStr;

/// Handles the `/flag` command.
///
/// This system handles the `/flag` command, and sends
/// out the the `PlayerFlag` packet when it does so.
#[derive(Default)]
pub struct Flag;

#[derive(SystemDataCustom)]
pub struct FlagData<'a> {
	conns: Connections<'a>,
	flags: WriteStorage<'a, FlagCode>,
}

impl EventHandlerTypeProvider for Flag {
	type Event = CommandEvent;
}

impl<'a> EventHandler<'a> for Flag {
	type SystemData = FlagData<'a>;

	fn on_event(&mut self, evt: &CommandEvent, data: &mut FlagData) {
		let &(conn, ref packet) = evt;

		let player = match data.conns.associated_player(conn) {
			Some(p) => p,
			None => return,
		};

		if packet.com != "flag" {
			return;
		}

		let flag = FlagCode::from_str(&packet.data).unwrap_or(FlagCode::UnitedNations);

		data.conns.send_to_all(PlayerFlag {
			id: player.into(),
			flag: flag,
		});

		data.flags.insert(player, flag).unwrap();
	}
}

system_info! {
	impl SystemInfo for Flag {
		type Dependencies = PacketHandler;
	}
}

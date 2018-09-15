use specs::*;
use types::*;

use component::event::*;
use protocol::server::PlayerFlag;
use protocol::FlagCode;

use systems::PacketHandler;
use utils::{EventHandler, EventHandlerTypeProvider};
use SystemInfo;

use std::str::FromStr;

/// Handles the `/flag` command.
///
/// This system handles the `/flag` command, and sends
/// out the the `PlayerFlag` packet when it does so.
#[derive(Default)]
pub struct Flag;

#[derive(SystemData)]
pub struct FlagData<'a> {
	pub conns: Read<'a, Connections>,
	pub flags: WriteStorage<'a, FlagCode>,
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

impl SystemInfo for Flag {
	type Dependencies = PacketHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

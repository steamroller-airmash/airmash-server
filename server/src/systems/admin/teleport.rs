use specs::*;
use types::*;

use std::option::NoneError;

use component::event::*;
use protocol::server::CommandReply;
use protocol::CommandReplyType;
use systems::PacketHandler;
use SystemInfo;

use utils::{EventHandler, EventHandlerTypeProvider};

use serde_json;

/// Directly set the position of an entity
#[derive(Default)]
pub struct Teleport;

#[derive(SystemData)]
pub struct TeleportData<'a> {
	pub entities: Entities<'a>,
	pub pos: WriteStorage<'a, Position>,
	pub conns: Read<'a, Connections>,
}

impl EventHandlerTypeProvider for Teleport {
	type Event = CommandEvent;
}

impl<'a> EventHandler<'a> for Teleport {
	type SystemData = TeleportData<'a>;

	fn on_event(&mut self, evt: &CommandEvent, data: &mut Self::SystemData) {
		let &(conn, ref packet) = evt;

		let player = match data.conns.associated_player(conn) {
			Some(p) => p,
			None => return,
		};

		if packet.com != "teleport" {
			return;
		}

		let result = parse_command_data(&packet.data).and_then(|x| {
			if x.id == 0 {
				return Ok((player, x));
			}

			let ent = data.entities.entity(x.id as u32);

			if !data.entities.is_alive(ent) {
				return Err(CommandParseError::NotAnEntity(x.id));
			}

			return Ok((ent, x));
		});

		if result.is_err() {
			data.conns.send_to(
				conn,
				CommandReply {
					ty: CommandReplyType::ShowInPopup,
					text: format!(
						"{}",
						serde_json::to_string_pretty(&result.unwrap_err()).unwrap()
					),
				},
			);
			return;
		}

		let (target, command_data) = result.unwrap();

		if let Some(pos) = data.pos.get_mut(target) {
			*pos = Position::new(command_data.pos_x, command_data.pos_y);
		}
	}
}

impl SystemInfo for Teleport {
	type Dependencies = PacketHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum CommandParseError<'a> {
	MissingArguments,
	IdNotANumber(&'a str),
	PositionNotANumber(&'a str),
	NotAnEntity(u16),
	OutOfBounds(f32),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ParsedCommand {
	pub id: u16,
	pub pos_x: f32,
	pub pos_y: f32,
}

impl<'a> From<NoneError> for CommandParseError<'a> {
	fn from(_: NoneError) -> Self {
		CommandParseError::MissingArguments
	}
}

fn split_strs<'a, I>(mut iter: I) -> Option<[&'a str; 3]>
where
	I: Iterator<Item = &'a str>,
{
	Some([iter.next()?, iter.next()?, iter.next()?])
}

fn parse_command_data<'a>(s: &'a str) -> Result<ParsedCommand, CommandParseError<'a>> {
	use self::CommandParseError::*;

	let strs = split_strs(s.split(" "))?;

	let com = ParsedCommand {
		id: strs[0].parse().map_err(|_| IdNotANumber(strs[0]))?,
		pos_x: strs[1].parse().map_err(|_| PositionNotANumber(strs[1]))?,
		pos_y: strs[2].parse().map_err(|_| PositionNotANumber(strs[2]))?,
	};

	if com.pos_x < -16384.0 || com.pos_x > 16384.0 {
		return Err(OutOfBounds(com.pos_x));
	}
	if com.pos_y < -8192.0 || com.pos_y > 8192.0 {
		return Err(OutOfBounds(com.pos_y));
	}

	return Ok(com);
}

#[cfg(test)]
mod test {
	use self::CommandParseError::*;
	use super::*;

	#[test]
	fn split_missing() {
		assert_eq!(split_strs("a b".split(" ")), None);
		assert_eq!(split_strs("a".split(" ")), None);
		assert_eq!(split_strs("".split(" ")), None);
	}

	#[test]
	fn split_3() {
		assert_eq!(split_strs("a b c".split(" ")), Some(["a", "b", "c"]));
	}

	#[test]
	fn split_4() {
		assert_eq!(split_strs("a b c d".split(" ")), Some(["a", "b", "c"]));
	}

	#[test]
	fn parse_valid() {
		assert_eq!(
			parse_command_data("1 5.0 0.0"),
			Ok(ParsedCommand {
				id: 1,
				pos_x: 5.0,
				pos_y: 0.0
			})
		);
	}

	#[test]
	fn parse_invalid_id() {
		assert_eq!(parse_command_data("foo 5.0 0.0"), Err(IdNotANumber("foo")));
		// Only return an error for the first one
		assert_eq!(parse_command_data("foo bar baz"), Err(IdNotANumber("foo")));
	}

	#[test]
	fn parse_invalid_coord() {
		assert_eq!(
			parse_command_data("1 foo 100.0"),
			Err(PositionNotANumber("foo"))
		);
		assert_eq!(
			parse_command_data("1 100.0 foo"),
			Err(PositionNotANumber("foo"))
		);
		// Only return an error for the first one
		assert_eq!(
			parse_command_data("1 foo bar"),
			Err(PositionNotANumber("foo"))
		);
	}

	#[test]
	fn parse_out_of_bounds_coord() {
		assert_eq!(
			parse_command_data("0 1000000.0 0.0"),
			Err(OutOfBounds(1000000.0))
		);
		assert_eq!(
			parse_command_data("0 0.0 1000000.0"),
			Err(OutOfBounds(1000000.0))
		);

		assert_eq!(
			parse_command_data("0 -1000000.0 0.0"),
			Err(OutOfBounds(-1000000.0))
		);
		assert_eq!(
			parse_command_data("0 0.0 -1000000.0"),
			Err(OutOfBounds(-1000000.0))
		);
	}
}

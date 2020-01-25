use crate::component::{Distance, Position};
use crate::ecs::prelude::*;
use crate::protocol::{client::Command, server::CommandReply, CommandReplyType};
use crate::resource::{packet::ClientPacket, Config};
use crate::sysdata::Connections;

use phf::Map;

use std::borrow::Cow;
use std::option::NoneError;

/// Admin command to teleport a named entity anywhere within
/// the map.
///
/// This can be used to teleport any entity with a position.
/// The only exception is that an id of 0 is interpreted
/// as teleporting the player that issued the command.
#[event_handler]
fn teleport<'a>(
    evt: &ClientPacket<Command<'static>>,
    entities: &Entities<'a>,
    pos: &mut WriteStorage<'a, Position>,
    config: &Read<'a, Config>,
    conns: &Connections<'a>,
) {
    if !config.admin_enabled {
        return;
    }

    if evt.packet.com != "teleport" {
        return;
    }

    let player = match conns.player(evt.connection) {
        Ok(Some(p)) => p,
        _ => return,
    };

    let result = parse_command_data(&evt.packet.data).and_then(|data| {
        if data.id == 0 {
            return Ok((player, data));
        }

        let entity = entities.forge(data.id as u32);

        if !entities.is_alive(entity) {
            return Err(CommandParseError::NotAnEntity(data.id));
        }

        Ok((entity, data))
    });

    let (target, command_data) = match result {
        Ok(x) => x,
        Err(e) => {
            let reply = CommandReply {
                ty: CommandReplyType::ShowInConsole,
                text: Cow::Owned(
                    serde_json::to_string_pretty(&e).expect("Failed to serialize error message"),
                ),
            };

            conns.send_to(evt.connection, reply);
            return;
        }
    };

    if let Some(pos) = pos.get_mut(target) {
        *pos = Position::new(command_data.pos_x, command_data.pos_y);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum CommandParseError<'a> {
    MissingArguments,
    IdNotANumber(&'a str),
    PositionNotANumber(&'a str),
    NotAnEntity(u16),
    OutOfBounds(f32),
    InvalidName,
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

fn split_strs<'a, I>(iter: I) -> Option<Vec<&'a str>>
where
    I: Iterator<Item = &'a str>,
{
    let strs = iter.take(3).collect::<Vec<&str>>();
    if strs.len() >= 2 {
        return Some(strs);
    }
    return None;
}

macro_rules! pos {
    ($x:expr, $y:expr) => {
        Position {
            x: Distance::new($x),
            y: Distance::new($y),
        }
    };
}

static NAMED_POSITIONS: Map<&str, Position> = phf_map! {
    "blue-flag" => pos!(-9670.0, -1470.0),
    "red-flag" => pos!(8600.0, -940.0),
    "greenland-spa-and-lounge" => pos!(-5000.0, -7000.0),
    "greenland" => pos!(-5000.0, -7000.0),
    "crimea" => pos!(2724.0, -2321.0),
    // The exact origin of how this name was
    // determined is shrouded in mystery.
    "mt-detect" => pos!(3550.0, -850.0),
    "red-spawn" => pos!(8600.0, -960.0),
    "blue-spawn" => pos!(-9670.0, -1470.0),
};

fn parse_command_data<'a>(s: &'a str) -> Result<ParsedCommand, CommandParseError<'a>> {
    use self::CommandParseError::*;

    let strs = split_strs(s.split(" "))?;

    let com = if strs.len() == 3 {
        ParsedCommand {
            id: strs[0].parse().map_err(|_| IdNotANumber(strs[0]))?,
            pos_x: strs[1].parse().map_err(|_| PositionNotANumber(strs[1]))?,
            pos_y: strs[2].parse().map_err(|_| PositionNotANumber(strs[2]))?,
        }
    } else if NAMED_POSITIONS.contains_key(strs[1]) {
        let position = NAMED_POSITIONS.get(strs[1]).unwrap();
        ParsedCommand {
            id: strs[0].parse().map_err(|_| IdNotANumber(strs[0]))?,
            pos_x: position.x.inner(),
            pos_y: position.y.inner(),
        }
    } else {
        return Err(InvalidName);
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
        assert_eq!(split_strs("a".split(" ")), None);
        assert_eq!(split_strs("".split(" ")), None);
    }

    #[test]
    fn split_3() {
        assert_eq!(split_strs("a b c".split(" ")), Some(vec!["a", "b", "c"]));
    }

    #[test]
    fn split_4() {
        assert_eq!(split_strs("a b c d".split(" ")), Some(vec!["a", "b", "c"]));
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

        assert_eq!(
            parse_command_data("1 blue-flag"),
            Ok(ParsedCommand {
                id: 1,
                pos_x: -9670.0,
                pos_y: -1470.0,
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
    fn parse_invalid_name() {
        assert_eq!(parse_command_data("0 orange-flag"), Err(InvalidName));
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

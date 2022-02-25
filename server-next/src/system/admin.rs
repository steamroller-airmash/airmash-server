use bstr::BString;
use smallvec::SmallVec;

use crate::component::*;
use crate::protocol::client::Command;
use crate::protocol::{server::CommandReply, CommandReplyType};
use crate::{event::PacketEvent, resource::Config};
use crate::{AirmashGame, Vector2};

#[handler]
fn teleport(event: &PacketEvent<Command>, game: &mut AirmashGame) {
  #[derive(Copy, Clone, Debug, PartialEq)]
  pub struct ParsedCommand {
    pub id: Option<u16>,
    pub pos_x: f32,
    pub pos_y: f32,
  }

  fn named_positions(s: &[u8]) -> Option<Vector2<f32>> {
    let (x, y) = match s {
      b"blue-flag" => (-9670.0, -1470.0),
      b"red-flag" => (8600.0, -940.0),
      b"greenland-spa-and-lounge" => (-5000.0, -7000.0),
      b"greenland" => (-5000.0, -7000.0),
      b"crimea" => (2724.0, -2321.0),
      // The exact origin of how this name was
      // determined is shrouded in mystery.
      b"mt-detect" => (3550.0, -850.0),
      b"red-spawn" => (7818.0, -2930.0),
      b"blue-spawn" => (-8878.0, -2971.0),
      _ => return None,
    };

    Some(Vector2::new(x, y))
  }

  fn parse_command_data(s: &BString) -> Result<ParsedCommand, String> {
    let args: SmallVec<[_; 3]> = s.split(|&x| x == b' ').collect();

    fn parse_arg<T: std::str::FromStr>(bytes: &[u8], err: &'static str) -> Result<T, &'static str> {
      std::str::from_utf8(bytes)
        .map_err(|_| err)?
        .parse()
        .map_err(|_| err)
    }

    let id = match parse_arg(args[0], "Player ID was not a number")? {
      0 => None,
      x => Some(x),
    };

    let command = if args.len() == 3 {
      ParsedCommand {
        id,
        pos_x: parse_arg(args[1], "Couldn't parse position")?,
        pos_y: parse_arg(args[2], "Couldn't parse position")?,
      }
    } else {
      let pos = match named_positions(args[1]) {
        Some(pos) => pos,
        None => return Err("Unknown named position".to_string()),
      };

      ParsedCommand {
        id,
        pos_x: pos.x,
        pos_y: pos.y,
      }
    };

    if command.pos_x.abs() > 16384.0 {
      return Err(format!("{} is out of bounds", command.pos_x));
    }
    if command.pos_y.abs() > 8192.0 {
      return Err(format!("{} is out of bounds", command.pos_y));
    }

    Ok(command)
  }

  if !game.resources.read::<Config>().admin_enabled {
    return;
  }

  if event.packet.com != "teleport" {
    return;
  }

  let command = match parse_command_data(&event.packet.data) {
    Ok(command) => command,
    Err(e) => {
      game.send_to(
        event.entity,
        CommandReply {
          ty: CommandReplyType::ShowInConsole,
          text: e.into(),
        },
      );
      return;
    }
  };

  let target = match command.id {
    Some(id) => match game.find_entity_by_id(id) {
      Some(ent) => ent,
      None => {
        game.send_to(
          event.entity,
          CommandReply {
            ty: CommandReplyType::ShowInConsole,
            text: "Unknown entity".into(),
          },
        );
        return;
      }
    },
    None => event.entity,
  };

  let start_time = game.start_time();

  if let Ok(mut pos) = game.world.get_mut::<Position>(target) {
    pos.x = command.pos_x;
    pos.y = command.pos_y;
  }

  // If we've teleported a player then have an update happen right away.
  if let Ok(mut last_update) = game.world.get_mut::<LastUpdateTime>(target) {
    last_update.0 = start_time;
  }
}

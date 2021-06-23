use serde_json;
use specs::*;

use crate::component::event::*;
use crate::protocol::server::CommandReply;
use crate::protocol::CommandReplyType;
use crate::systems::PacketHandler;
use crate::types::*;

use crate::utils::{EventHandler, EventHandlerTypeProvider};

/// Print some properties of entities in-game
#[derive(Default)]
pub struct DebugPrint;

#[derive(SystemData)]
pub struct DebugPrintData<'a> {
  entities: Entities<'a>,
  config: Read<'a, Config>,
  conns: Read<'a, Connections>,

  pos: ReadStorage<'a, Position>,
  vel: ReadStorage<'a, Velocity>,
  rot: ReadStorage<'a, Rotation>,
  health: ReadStorage<'a, Health>,
  energy: ReadStorage<'a, Energy>,
}

impl EventHandlerTypeProvider for DebugPrint {
  type Event = CommandEvent;
}

impl<'a> EventHandler<'a> for DebugPrint {
  type SystemData = DebugPrintData<'a>;

  fn on_event(&mut self, evt: &CommandEvent, data: &mut Self::SystemData) {
    let &(conn, ref packet) = evt;

    if !data.config.admin_enabled {
      return;
    }

    let player = match data.conns.associated_player(conn) {
      Some(p) => p,
      None => return,
    };

    if packet.com != "debug" {
      return;
    }

    let res = parse_command(&packet.data).and_then(|x| {
      if x.id == 0 {
        return Ok((player, x.ty));
      }

      let ent = data.entities.entity(x.id as u32);
      if !data.entities.is_alive(ent) {
        return Err(ParseError::NotAnEntity(x.id));
      }

      Ok((ent, x.ty))
    });

    if res.is_err() {
      data.conns.send_to(
        conn,
        CommandReply {
          ty: CommandReplyType::ShowInConsole,
          text: serde_json::to_string_pretty(&res.unwrap_err()).unwrap(),
        },
      );
      return;
    }

    let (target, ty) = res.unwrap();

    let formatted = match ty {
      "position" => format!("{:?}", data.pos.get(target)),
      "velocity" => format!("{:?}", data.vel.get(target)),
      "rotation" => format!("{:?}", data.rot.get(target)),
      "health" => format!("{:?}", data.health.get(target)),
      "energy" => format!("{:?}", data.energy.get(target)),
      _ => format!("no such printable component"),
    };

    data.conns.send_to(
      conn,
      CommandReply {
        ty: CommandReplyType::ShowInConsole,
        text: formatted,
      },
    );
  }
}

#[derive(Serialize, Debug)]
enum ParseError<'a> {
  NotEnoughArguments,
  IdNotANumber(&'a str),
  NotAnEntity(u16),
}

struct ParsedCommand<'a> {
  id: u16,
  ty: &'a str,
}

fn parse_command<'a>(s: &'a str) -> Result<ParsedCommand<'a>, ParseError<'a>> {
  use self::ParseError::*;

  let strs = s.split(" ").take(2).collect::<Vec<&str>>();
  if strs.len() < 2 {
    return Err(NotEnoughArguments);
  }

  let id = strs[0].parse().map_err(|_| IdNotANumber(strs[0]))?;
  let ty = strs[1];

  Ok(ParsedCommand { id, ty })
}

system_info! {
  impl SystemInfo for DebugPrint {
    type Dependencies = PacketHandler;
  }
}

use std::borrow::Cow;
use std::time::Duration;

use airmash::component::*;
use airmash::event::{PacketEvent, PlayerRespawn};
use airmash::protocol::client::Command;
use airmash::AirmashGame;

#[handler]
fn switch_teams(event: &PacketEvent<Command>, game: &mut AirmashGame) {
  use airmash::protocol::server::{CommandReply, Error, PlayerReteam, PlayerReteamPlayer};
  use airmash::protocol::ErrorType;

  use crate::config::{BLUE_TEAM, RED_TEAM};

  if event.packet.com != "switch" {
    return;
  }

  let this_frame = game.this_frame();
  let (team, name, &alive, &last_action, _) =
    match game
      .world
      .query_one_mut::<(&mut Team, &Name, &IsAlive, &LastActionTime, &IsPlayer)>(event.entity)
    {
      Ok(query) => query,
      Err(_) => return,
    };

  if this_frame - last_action.0 < Duration::from_secs(2) {
    game.send_to(
      event.entity,
      Error {
        error: ErrorType::IdleRequiredBeforeRespawn,
      },
    );
    return;
  }

  team.0 = match team.0 {
    RED_TEAM => BLUE_TEAM,
    BLUE_TEAM => RED_TEAM,
    x => x,
  };

  let reteam = PlayerReteam {
    players: vec![PlayerReteamPlayer {
      id: event.entity.id() as _,
      team: team.0,
    }],
  };

  let reply = CommandReply {
    ty: airmash::protocol::CommandReplyType::ShowInConsole,
    text: format!(
      "{} has switched to {}",
      name.0,
      match team.0 {
        RED_TEAM => "red team".into(),
        BLUE_TEAM => "blue team".into(),
        _ => Cow::Owned(format!("team {}", team.0)),
      }
    )
    .into(),
  };

  game.send_to_all(reteam);
  game.send_to_all(reply);

  game.dispatch(PlayerRespawn {
    player: event.entity,
    alive: alive.0,
  });
}

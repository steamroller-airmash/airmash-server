use smallvec::SmallVec;

use crate::component::*;
use crate::event::PlayerSpectate;
use crate::util::NalgebraExt;
use crate::{AirmashGame, Vector2};

#[handler(priority = crate::priority::MEDIUM)]
fn update_player(event: &PlayerSpectate, game: &mut AirmashGame) {
  let (spec, alive, _) = match game
    .world
    .query_one_mut::<(&mut IsSpectating, &mut IsAlive, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  spec.0 = true;
  alive.0 = false;
}

#[handler]
fn send_packets(event: &PlayerSpectate, game: &mut AirmashGame) {
  use crate::protocol::server::{GameSpectate, PlayerKill};

  let (&pos, &target, _) = match game
    .world
    .query_one_mut::<(&Position, &Spectating, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  // If the player was already dead then we don't need to despawn their plane.
  if event.was_alive {
    game.send_to_visible(
      pos.0,
      PlayerKill {
        id: event.player.id() as _,
        killer: None,
        pos: Vector2::zeros().into(),
      },
    );
  }

  game.send_to(
    event.player,
    GameSpectate {
      id: target.0.unwrap_or(event.player).id() as _,
    },
  );
}

#[handler]
fn retarget_spectators(event: &PlayerSpectate, game: &mut AirmashGame) {
  use crate::util::spectate::*;

  let mut query = game.world.query::<&mut Spectating>().with::<IsPlayer>();
  let mut events = SmallVec::<[_; 8]>::new();
  for (ent, spec) in query.iter() {
    if spec.0 == Some(event.player) {
      spec.0 = match spectate_target(ent, spec.0, SpectateTarget::Next, game) {
        Some(ent) if ent == event.player => Some(ent),
        x => x,
      };

      events.push(PlayerSpectate {
        player: ent,
        was_alive: false,
      });
    }
  }

  drop(query);

  game.dispatch_many(events);
}

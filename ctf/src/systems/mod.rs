use airmash::event::{EventStealth, PacketEvent, PlayerKilled, PlayerLeave, PlayerRespawn};
use airmash::protocol::client::Command;
use airmash::{AirmashGame, Entity};

mod command;
mod on_flag_event;
mod on_frame;
mod on_game_end;
mod on_game_start;
mod on_player_join;
mod on_player_leave;
mod on_player_respawn;

pub fn drop_carried_flags(player: Entity, game: &mut AirmashGame) {
  use airmash::component::IsPlayer;
  use smallvec::SmallVec;

  use crate::component::{FlagCarrier, IsFlag};
  use crate::event::FlagEvent;

  if game.world.get::<IsPlayer>(player).is_err() {
    return;
  }

  let query = game.world.query_mut::<&FlagCarrier>().with::<IsFlag>();

  let mut events = SmallVec::<[_; 2]>::new();
  for (flag, carrier) in query {
    if carrier.0 != Some(player) {
      continue;
    }

    events.push(FlagEvent {
      ty: crate::event::FlagEventType::Drop,
      player: Some(player),
      flag,
    })
  }

  game.dispatch_many(events);
}

#[handler]
fn drop_on_leave(event: &PlayerLeave, game: &mut AirmashGame) {
  drop_carried_flags(event.player, game);
}

#[handler]
fn drop_on_death(event: &PlayerKilled, game: &mut AirmashGame) {
  drop_carried_flags(event.player, game);
}

#[handler]
fn drop_on_stealth(event: &EventStealth, game: &mut AirmashGame) {
  drop_carried_flags(event.player, game);
}

#[handler]
fn drop_on_respawn(event: &PlayerRespawn, game: &mut AirmashGame) {
  drop_carried_flags(event.player, game);
}

#[handler]
fn drop_on_command(event: &PacketEvent<Command>, game: &mut AirmashGame) {
  if event.packet.com == "drop" {
    drop_carried_flags(event.entity, game);
  }
}

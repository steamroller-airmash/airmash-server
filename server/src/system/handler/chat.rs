use crate::component::{IsPlayer, Position, SpecialActive, Team};
use crate::config::PlanePrototypeRef;
use crate::event::PacketEvent;
use crate::protocol::client::{Chat, Say, TeamChat, Whisper};
use crate::protocol::server as s;
use crate::AirmashGame;

#[handler]
fn on_chat(event: &PacketEvent<Chat>, game: &mut AirmashGame) {
  if game.world.get::<IsPlayer>(event.entity).is_err() {
    return;
  }

  game.send_to_all(s::ChatPublic {
    id: event.entity.id() as _,
    text: event.packet.text.clone(),
  });
}

#[handler]
fn on_team_chat(event: &PacketEvent<TeamChat>, game: &mut AirmashGame) {
  if game.world.get::<IsPlayer>(event.entity).is_err() {
    return;
  }

  let team = game.world.get::<Team>(event.entity).unwrap();

  game.send_to_team(
    team.0,
    s::ChatTeam {
      id: event.entity.id() as _,
      text: event.packet.text.clone(),
    },
  );
}

#[handler]
fn on_whisper(event: &PacketEvent<Whisper>, game: &mut AirmashGame) {
  if game.world.get::<IsPlayer>(event.entity).is_err() {
    return;
  }

  let target = match game.find_entity_by_id(event.packet.id) {
    Some(entity) => entity,
    None => return,
  };

  if game.world.get::<IsPlayer>(target).is_err() {
    return;
  }

  let packet = s::ChatWhisper {
    to: target.id() as _,
    from: event.entity.id() as _,
    text: event.packet.text.clone(),
  };

  if event.entity != target {
    game.send_to(target, packet.clone());
  }
  game.send_to(event.entity, packet);
}

#[handler]
fn on_say(event: &PacketEvent<Say>, game: &mut AirmashGame) {
  let (&pos, &plane, &special, &team, _) = match game.world.query_one_mut::<(
    &Position,
    &PlanePrototypeRef,
    &SpecialActive,
    &Team,
    &IsPlayer,
  )>(event.entity)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  let packet = s::ChatSay {
    id: event.entity.id() as _,
    text: event.packet.text.clone(),
  };

  if plane.special.is_stealth() && special.0 {
    game.send_to_team_visible(team.0, pos.0, packet);
  } else {
    game.send_to_visible(pos.0, packet);
  }
}

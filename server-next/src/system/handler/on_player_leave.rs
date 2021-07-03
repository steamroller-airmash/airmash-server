use smallvec::SmallVec;

use crate::component::*;
use crate::event::PlayerLeave;
use crate::event::PlayerSpectate;
use crate::AirmashWorld;

#[handler]
fn send_packet(event: &PlayerLeave, game: &mut AirmashWorld) {
  use crate::protocol::server as s;

  if let Err(_) = game.world.get::<IsPlayer>(event.player) {
    return;
  }

  game.send_to_all(s::PlayerLeave {
    id: event.player.id() as _,
  });
}

#[handler]
fn remove_name(event: &PlayerLeave, game: &mut AirmashWorld) {
  use crate::resource::TakenNames;

  let mut taken_names = game.resources.write::<TakenNames>();
  let name = match game.world.get::<Name>(event.player) {
    Ok(name) => name,
    Err(_) => return,
  };

  taken_names.remove(&name.0);
}

#[handler]
fn retarget_spectators(event: &PlayerLeave, game: &mut AirmashWorld) {
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

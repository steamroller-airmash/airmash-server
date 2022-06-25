use crate::component::*;
use crate::config::PlanePrototypeRef;
use crate::event::{PlayerPowerup, PlayerRespawn, PlayerSpawn};
use crate::resource::Config;
use crate::util::NalgebraExt;
use crate::{AirmashGame, EntitySetBuilder, Vector2};

#[handler]
fn send_packet(event: &PlayerRespawn, game: &mut AirmashGame) {
  use crate::protocol::server::PlayerRespawn;

  let (&pos, &rot, upgrades, effects, _) =
    match game
      .world
      .query_one_mut::<(&Position, &Rotation, &Upgrades, &Effects, &IsAlive)>(event.player)
    {
      Ok(query) => query,
      Err(_) => return,
    };

  let packet = PlayerRespawn {
    id: event.player.id() as _,
    pos: pos.into(),
    rot: rot.0,
    upgrades: crate::util::get_server_upgrades(upgrades, effects),
  };

  game.send_to_entities(
    EntitySetBuilder::visible(game, pos.0).including(event.player),
    packet,
  );
}

// Set priority to be higher than PRE_LOGIN so that other handlers making
// changes don't have theirs get stomped over.
#[handler(priority = crate::priority::PRE_LOGIN)]
fn reset_player(event: &PlayerRespawn, game: &mut AirmashGame) {
  let config = game.resources.read::<Config>();

  let mut query = match game.world.query_one::<(
    &mut Position,
    &mut Velocity,
    &mut Rotation,
    &mut Health,
    &mut Energy,
    &mut HealthRegen,
    &mut EnergyRegen,
    &mut IsAlive,
    &mut IsSpectating,
    &mut SpecialActive,
    &mut KeyState,
    &mut Spectating,
    &PlanePrototypeRef,
  )>(event.player)
  {
    Ok(query) => query.with::<IsPlayer>(),
    Err(_) => return,
  };

  let (
    pos,
    vel,
    rot,
    health,
    energy,
    health_regen,
    energy_regen,
    alive,
    spectating,
    active,
    keystate,
    spectgt,
    &plane,
  ) = match query.get() {
    Some(query) => query,
    None => return,
  };

  pos.0 = Vector2::zeros();
  vel.0 = Vector2::zeros();
  rot.0 = 0.0;
  health.0 = 1.0;
  energy.0 = 1.0;
  health_regen.0 = plane.health_regen;
  energy_regen.0 = plane.energy_regen;
  *keystate = KeyState::default();
  alive.0 = true;
  spectating.0 = false;
  active.0 = false;
  spectgt.0 = None;

  let proto = config.powerups.get("spawn-shield").copied();

  drop(config);
  drop(query);

  if let Some(proto) = proto {
    game.dispatch(PlayerPowerup {
      player: event.player,
      powerup: proto,
    });
  }
}

#[handler]
fn dispatch_player_spawn(event: &PlayerRespawn, game: &mut AirmashGame) {
  game.dispatch(PlayerSpawn {
    player: event.player,
  });
}

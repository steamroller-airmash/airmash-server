use crate::component::*;
use crate::event::PlayerPowerup;
use crate::event::PlayerRespawn;
use crate::protocol::PlaneType;
use crate::protocol::PowerupType;
use crate::resource::Config;
use crate::Vector2;
use crate::{AirmashWorld, EntitySetBuilder};

#[handler]
fn send_packet(event: &PlayerRespawn, game: &mut AirmashWorld) {
  use crate::protocol::server::PlayerRespawn;

  let (&pos, &rot, upgrades, powerup, _) =
    match game
      .world
      .query_one_mut::<(&Position, &Rotation, &Upgrades, &Powerup, &IsAlive)>(event.player)
    {
      Ok(query) => query,
      Err(_) => return,
    };

  let packet = PlayerRespawn {
    id: event.player.id() as _,
    pos: pos.0,
    rot: rot.0,
    upgrades: crate::util::get_server_upgrades(upgrades, powerup),
  };

  game.send_to_entities(
    EntitySetBuilder::visible(game, pos.0).including(event.player),
    packet,
  );
}

#[handler(priority = crate::priority::MEDIUM)]
fn reset_player(event: &PlayerRespawn, game: &mut AirmashWorld) {
  let config = game.resources.read::<Config>();

  let mut query = match game.world.query_one::<(
    &mut Position,
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
    &PlaneType,
  )>(event.player)
  {
    Ok(query) => query.with::<IsPlayer>(),
    Err(_) => return,
  };

  let (
    pos,
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

  let info = &config.planes[plane];

  pos.0 = Vector2::zeros();
  rot.0 = 0.0;
  health.0 = 1.0;
  energy.0 = 1.0;
  health_regen.0 = info.health_regen;
  energy_regen.0 = info.energy_regen;
  *keystate = KeyState::default();
  alive.0 = true;
  spectating.0 = false;
  active.0 = false;
  spectgt.0 = None;

  let powerup = PlayerPowerup {
    player: event.player,
    ty: PowerupType::Shield,
    duration: config.spawn_shield_duration,
  };

  drop(config);
  drop(query);

  game.dispatch(powerup);
}

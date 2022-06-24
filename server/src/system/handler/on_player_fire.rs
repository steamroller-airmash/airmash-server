use crate::component::*;
use crate::config::MissilePrototypeRef;
use crate::event::PlayerFire;
use crate::resource::Config;
use crate::AirmashGame;

#[handler]
pub fn send_player_fire(event: &PlayerFire, game: &mut AirmashGame) {
  use crate::protocol::server as s;

  let config = game.resources.read::<Config>();
  let clock = crate::util::get_current_clock(game);

  let mut projectiles = Vec::with_capacity(event.missiles.len());
  for &missile in event.missiles.iter() {
    let mut query = match game
      .world
      .query_one::<(&Position, &Velocity, &MissilePrototypeRef)>(missile)
    {
      Ok(query) => query.with::<IsMissile>(),
      Err(_) => {
        warn!("Missile event contained bad missile entity {:?}", missile);
        continue;
      }
    };

    if let Some((pos, vel, &mob)) = query.get() {
      projectiles.push(s::PlayerFireProjectile {
        id: missile.id() as _,
        pos: pos.into(),
        speed: vel.into(),
        ty: mob.server_type,
        accel: (vel.normalized() * mob.accel).into(),
        max_speed: mob.max_speed,
      });
    } else {
      warn!("Missile {:?} missing required components", missile);
    }
  }

  let mut query = match game
    .world
    .query_one::<(&Position, &Energy, &EnergyRegen)>(event.player)
  {
    Ok(query) => query.with::<IsPlayer>(),
    Err(_) => return,
  };

  if let Some((&pos, energy, regen)) = query.get() {
    let packet = s::PlayerFire {
      clock,
      id: event.player.id() as _,
      energy: energy.0,
      energy_regen: regen.0,
      projectiles,
    };

    drop(query);
    drop(config);

    game.send_to_visible(pos.0, packet);
  } else {
    warn!("Player {:?} missing required components", event.player);
  }
}

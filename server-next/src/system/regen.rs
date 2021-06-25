use crate::component::*;
use crate::resource::{LastFrame, ThisFrame};
use crate::AirmashWorld;

pub fn update(game: &mut AirmashWorld) {
  run_energy_regen(game);
  run_health_regen(game);
}

fn run_energy_regen(game: &mut AirmashWorld) {
  let last_frame = game.resources.read::<LastFrame>().0;
  let this_frame = game.resources.read::<ThisFrame>().0;

  let query = game
    .world
    .query_mut::<(&mut Energy, &mut EnergyRegen)>()
    .with::<IsPlayer>();

  let delta = crate::util::convert_time(this_frame - last_frame);

  for (_, (energy, regen)) in query {
    energy.0 += regen.0 * delta;
    energy.0 = energy.0.clamp(0.0, 1.0);
  }
}

pub fn run_health_regen(game: &mut AirmashWorld) {
  let last_frame = game.resources.read::<LastFrame>().0;
  let this_frame = game.resources.read::<ThisFrame>().0;

  let query = game
    .world
    .query_mut::<(&mut Health, &mut HealthRegen)>()
    .with::<IsPlayer>();

  let delta = crate::util::convert_time(this_frame - last_frame);

  for (_, (health, regen)) in query {
    health.0 += regen.0 * delta;
    health.0 = health.0.clamp(0.0, 1.0);
  }
}

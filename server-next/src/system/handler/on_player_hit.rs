use crate::component::*;
use crate::event::PlayerHit;
use crate::AirmashGame;

#[handler]
fn update_damage(event: &PlayerHit, game: &mut AirmashGame) {
  let attacker = match event.attacker {
    Some(attacker) => attacker,
    None => return,
  };

  let (damage, _) = match game
    .world
    .query_one_mut::<(&mut TotalDamage, &IsPlayer)>(attacker)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  damage.0 += event.damage;
}

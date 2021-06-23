use hashbrown::HashMap;

use crate::protocol::MobType;
use crate::types::{Distance, Mob, Position};

use crate::consts::config::POWERUP_RADIUS;

lazy_static! {
  pub static ref COLLIDERS: HashMap<Mob, Vec<(Position, Distance)>> = {
    let mut map = HashMap::default();

    let missiles = [
      MobType::PredatorMissile,
      MobType::GoliathMissile,
      MobType::MohawkMissile,
      MobType::TornadoSingleMissile,
      MobType::TornadoTripleMissile,
      MobType::ProwlerMissile,
    ];

    let powerups = [MobType::Upgrade, MobType::Shield, MobType::Inferno];

    for val in missiles.into_iter() {
      map.insert(*val, vec![(Position::default(), Distance::new(1.0))]);
    }
    for val in powerups.into_iter() {
      map.insert(*val, vec![(Position::default(), POWERUP_RADIUS)]);
    }

    map
  };
}

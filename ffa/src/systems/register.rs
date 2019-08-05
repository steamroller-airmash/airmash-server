use super::*;
use crate::protocol::MobType;
use crate::types::*;
use airmash_server::{Builder, Distance, Position};
use specs::*;
use std::time::Duration;

pub fn register<'a, 'b>(world: &mut World, builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
    // inferno spawn point in Europe
    world.add_resource(PowerupSpawnPoints(vec![PowerupSpawnPoint {
        pos: Position::new(Distance::new(920.0), Distance::new(-2800.0)),
        powerup_type: MobType::Inferno,
        respawn_delay: Duration::from_secs(60),
        next_respawn_time: None,
        powerup_entity: None,
    }]));

    builder
        .with_handler::<AddDamage>()
        .with_handler::<TrackDamage>()
        .with_handler::<SendScoreDetailed>()
}

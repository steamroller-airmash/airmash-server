use airmash_server::{Builder, Position, Distance};
use protocol::MobType;
use specs::*;
use std::time::Duration;
use types::*;
use super::*;

pub fn register<'a, 'b>(world: &mut World, builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	// inferno spawn point in Europe
	world.add_resource(
		PowerupSpawnPoints(
			vec![
				PowerupSpawnPoint {
					pos: Position::new(
						Distance::new(920.0),
						Distance::new(-2800.0),
					),
					powerup_type: MobType::Inferno,
					respawn_delay: Duration::from_secs(60),
					next_respawn_time: None,
					powerup_entity: None
				}
	]));

    builder
        .with::<AddDamage>()
        .with::<TrackDamage>()
        .with::<SendScoreDetailed>()
}

use super::*;
use crate::dispatch::Builder;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	builder
		// Upkeep systems
		.with::<on_upkeep::CheckExpired>()
		.with::<on_upkeep::SpawnRandomPowerup>()
		.with::<on_upkeep::SpawnFixedPowerup>()
		// Collisions
		.with_handler::<on_collision::HandleCollision>()
		// Despawn handlers
		.with_handler::<on_despawn::Cleanup>()
		.with_handler::<on_despawn::SendPacket>()
		// Spawn handlers
		.with_handler::<on_spawn::SendPacket>()
		// Powerup expiry handlers
		.with_handler::<on_expire::ForceUpdate>()
}

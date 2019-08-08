use crate::systems::handlers;
use crate::systems::missile;
use crate::systems::*;

use crate::dispatch::Builder;

pub fn register<'a, 'b>(disp: Builder<'a, 'b>) -> Builder<'a, 'b> {
	disp.with::<run_futures::RunTimedFutures>()
		// TODO: This should probably be done outside of the
		//       register function since it is required for
		//       the task system.
		.with::<TaskTimerSystem>()
		// Other handlers
		.with_registrar(handlers::register)
		// Systems with dependencies on handlers
		.with::<PositionUpdate>()
		// Register missle handlers
		.with_registrar(missile::register)
		// EnergyRegen depends on MissileHit
		.with::<EnergyRegenSystem>()
		.with::<HealthRegenSystem>()
		.with::<Disconnect>()
		// Collision handling
		.with_registrar(collision::register)
		// Specials
		.with_registrar(specials::register)
		// Limiters
		.with_registrar(limiting::register)
		// Upgrades
		.with_registrar(upgrades::register)
		// Admin/Debug Commands
		.with_registrar(admin::register)
		// Powerups
		.with_registrar(powerups::register)
		// Visibility
		.with_registrar(visibility::register)
		// Timers
		.with_registrar(timers::register)
		// Server statistics
		.with_registrar(stats::register)
}

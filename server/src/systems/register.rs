use systems::handlers;
use systems::missile;
use systems::*;

use dispatch::Builder;

pub fn register<'a, 'b>(disp: Builder<'a, 'b>) -> Builder<'a, 'b> {
	disp
		.with::<run_futures::RunTimedFutures>()
		// Other handlers
		.with_registrar(handlers::register)
		// Systems with dependencies on handlers
		.with::<PositionUpdate>()
		// Register missle handlers
		.with_registrar(missile::register)
		// EnergyRegen depends on MissileHit
		.with::<EnergyRegenSystem>()
		.with::<HealthRegenSystem>()
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
}

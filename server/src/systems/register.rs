use handlers;
use systems::missile;
use systems::*;

use dispatch::Builder;

pub fn register<'a, 'b>(disp: Builder<'a, 'b>) -> Builder<'a, 'b> {
	let disp = disp
		// Add handlers here
		.with::<handlers::OnOpenHandler>()
		.with::<handlers::OnCloseHandler>()
		.with::<handlers::LoginHandler>()
		.with::<handlers::KeyHandler>()
		.with::<handlers::ChatHandler>()
		.with::<handlers::SayHandler>()
		.with::<handlers::PongHandler>()
		.with::<handlers::ScoreBoardTimerHandler>()
		.with::<handlers::PingTimerHandler>()
		.with::<handlers::CommandHandler>()
		.with::<handlers::SignalHandler>()

		// Systems with dependencies on handlers
		.with::<PositionUpdate>();

	let disp = missile::register(disp)
		// EnergyRegen depends on MissileHit
		.with::<EnergyRegenSystem>()
		.with::<HealthRegenSystem>();
	collision::register(disp)
}

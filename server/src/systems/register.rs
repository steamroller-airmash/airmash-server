use systems::handlers;
use systems::missile;
use systems::*;

use dispatch::Builder;

pub fn register<'a, 'b>(disp: Builder<'a, 'b>) -> Builder<'a, 'b> {
	let disp = disp
		.with::<run_futures::RunTimedFutures>()

		// Add handlers here
		.with::<handlers::packet::OnOpenHandler>()
		.with::<handlers::packet::OnCloseHandler>()
		.with::<handlers::packet::LoginHandler>()
		.with::<handlers::packet::KeyHandler>()
		.with::<handlers::packet::ChatHandler>()
		.with::<handlers::packet::SayHandler>()
		.with::<handlers::packet::PongHandler>()
		.with::<handlers::packet::ScoreBoardTimerHandler>()
		.with::<handlers::packet::PingTimerHandler>()
		.with::<handlers::packet::CommandHandler>()
		.with::<handlers::packet::SignalHandler>()
		.with::<handlers::packet::WhisperHandler>()

		// Systems with dependencies on handlers
		.with::<PositionUpdate>();

	let disp = missile::register(disp)
		// EnergyRegen depends on MissileHit
		.with::<EnergyRegenSystem>()
		.with::<HealthRegenSystem>();
	// Spectate handling
	let disp = spectate::register(disp);

	// Other handlers
	let disp = handlers::register(disp);
	// Specials
	let disp = specials::register(disp);

	// Collision handling
	collision::register(disp)
}

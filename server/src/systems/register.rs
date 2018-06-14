use handlers;
use specs::*;
use systems::*;

pub fn register<'a, 'b>(
	_: &mut World,
	disp: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
	disp
		// Add handlers here
		.with(handlers::OnOpenHandler::new(),  "onopen",  &["packet"])
		.with(handlers::OnCloseHandler::new(), "onclose", &["onopen"])
		.with(handlers::LoginHandler::new(),   "onlogin", &["onclose"])
		.with(handlers::KeyHandler::new(),     "onkey",   &["onclose"])
		.with(handlers::ChatHandler::new(),    "onchat",  &["onclose"])
		.with(handlers::SayHandler::new(),     "onsay",   &["onclose"])
		.with(handlers::PongHandler::new(),    "onpong",  &["onclose"])
		.with(handlers::ScoreBoardTimerHandler::new(), "scoreboard", &["timer"])
		.with(handlers::PingTimerHandler::new(),  "ping",  &["timer"])
		.with(handlers::CommandHandler::new(),    "command", &["onclose"])
		.with(handlers::SignalHandler::default(), "handler", &[])

		// Systems with dependencies on handlers
		.with(PositionUpdate::new(),       "position_update", &["onkey"])
		.with(MissileFireHandler{},        "missile_fire",    &["position_update"])
		.with(PlaneCollisionSystem::new(), "collisions",      &["position_update"])
		.with(BounceSystem::new(),         "bounces",         &["collisions"])
		.with(MissileUpdate{},             "missile_update",  &["missile_fire"])
		.with(EnergyRegenSystem{},         "energy_regen",    &["missile_fire"])
}


use specs::*;
use systems;
use handlers;

pub fn register<'a, 'b>(_: &mut World, disp: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
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
		.with(systems::PositionUpdate::new(),  "position_update", &["onkey"])
		.with(systems::MissileFireHandler{},   "missile_fire", &["position_update"])
		.with(systems::CollisionSystem::new(), "collisions", &["position_update"])
		.with(systems::BounceSystem::new(),    "bounces",    &["collisions"])
		.with(systems::MissileUpdate{},        "missile_update", &["missile_fire"])
}

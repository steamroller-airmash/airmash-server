use systems;

pub type AllEventHandlers = ();
pub type KnownEventSources = (
	systems::missile::MissileCull,
	systems::handlers::game::on_player_hit::CreateDespawnEvent,
);

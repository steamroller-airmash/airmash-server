use systems::handlers::game::*;

pub type AllDespawnHandlers = ();

pub type KnownEventSources = (
	on_player_killed::CreateDespawnEvent,
	on_spectate_event::CreateDespawnEvent,
	on_leave::CreateDespawnEvent,
);

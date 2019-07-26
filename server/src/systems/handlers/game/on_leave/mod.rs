mod create_despawn_event;
mod free_name;
mod send_packet;
mod update_players_game;

pub use self::create_despawn_event::CreateDespawnEvent;
pub use self::free_name::FreeName;
pub use self::send_packet::SendPlayerLeave;
pub use self::update_players_game::UpdatePlayersGame;

pub type AllLeaveHandlers = (
	CreateDespawnEvent,
	FreeName,
	UpdatePlayersGame,
	SendPlayerLeave,
);

pub type KnownEventSources = (crate::systems::handlers::packet::OnCloseHandler);

mod create_despawn_event;
mod free_name;
mod update_players_game;

pub use self::create_despawn_event::CreateDespawnEvent;
pub use self::free_name::FreeName;
pub use self::update_players_game::UpdatePlayersGame;

pub type AllLeaveHandlers = (CreateDespawnEvent, FreeName, UpdatePlayersGame);

use systems;

pub type KnownEventSources = (systems::handlers::packet::OnCloseHandler);

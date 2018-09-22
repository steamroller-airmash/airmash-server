mod create_despawn_event;
mod reset_keystate;
mod send_player_respawn;
mod set_traits;

pub use self::create_despawn_event::CreateDespawnEvent;
pub use self::reset_keystate::ResetKeyState;
pub use self::send_player_respawn::SendPlayerRespawn;
pub use self::set_traits::SetTraits;

pub type AllRespawnHandlers = (
	ResetKeyState,
	SendPlayerRespawn,
	SetTraits,
	CreateDespawnEvent,
);

use systems;

pub type KnownEventSources = (systems::handlers::command::Respawn);

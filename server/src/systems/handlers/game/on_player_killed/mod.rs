mod create_despawn_event;
mod display_message;
mod set_respawn_timer;
mod update_score;

pub use self::create_despawn_event::CreateDespawnEvent;
pub use self::display_message::DisplayMessage;
pub use self::set_respawn_timer::SetRespawnTimer;
pub use self::update_score::UpdateScore;

use systems;

pub type PlayerKilledHandlers = (
	DisplayMessage,
	SetRespawnTimer,
	UpdateScore,
	CreateDespawnEvent,
);

pub type KnownEventSources = (systems::handlers::game::on_player_hit::InflictDamage);

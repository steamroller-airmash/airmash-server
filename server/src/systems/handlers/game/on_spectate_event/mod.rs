mod create_despawn_event;
mod send_kill_packet;
mod send_spectate_packet;
mod send_timer_event;
mod set_spectate_flag;
mod set_target;

pub use self::create_despawn_event::CreateDespawnEvent;
pub use self::send_kill_packet::SendKillPacket;
pub use self::send_spectate_packet::SendSpectatePacket;
pub use self::send_timer_event::SendTimerEvent;
pub use self::set_spectate_flag::SetSpectateFlag;
pub use self::set_target::SetSpectateTarget;

pub type AllSpectateEventHandlers = (
	SendKillPacket,
	SendSpectatePacket,
	SendTimerEvent,
	SetSpectateFlag,
	SetSpectateTarget,
	CreateDespawnEvent,
);

use systems;

pub type KnownEventSources = (systems::handlers::command::Spectate);

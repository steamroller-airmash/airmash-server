use systems;

mod send_missile_update;
mod send_powerup_update;

pub use self::send_missile_update::SendMissileUpdate;
pub use self::send_powerup_update::SendPowerupUpdate;

pub type AllEventHandlers = (SendMissileUpdate, SendPowerupUpdate);
pub type KnownEventSources = (systems::visibility::TrackVisible);

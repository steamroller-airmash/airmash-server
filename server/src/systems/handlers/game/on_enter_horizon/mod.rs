use systems;

mod send_missile_update;

pub use self::send_missile_update::SendMissileUpdate;

pub type AllEventHandlers = (SendMissileUpdate);
pub type KnownEventSources = (systems::visibility::TrackVisible);

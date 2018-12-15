use systems;

mod send_leave_horizon;

pub use self::send_leave_horizon::SendLeaveHorizon;

pub type AllEventHandlers = (SendLeaveHorizon);
pub type KnownEventSources = (systems::visibility::TrackVisible);

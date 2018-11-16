mod force_update;

pub use self::force_update::ForceUpdate;

pub type AllEventHandlers = (
	ForceUpdate,
);

pub type KnownEventSources = ();

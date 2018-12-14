use systems;

pub type AllEventHandlers = ();
pub type KnownEventSources = (systems::missile::MissileCull, systems::missile::MissileHit);

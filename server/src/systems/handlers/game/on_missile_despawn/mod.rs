use systems;

mod send_mob_despawn;
mod send_mob_despawn_coords;

pub use self::send_mob_despawn::SendMobDespawn;
pub use self::send_mob_despawn_coords::SendMobDespawnCoords;

pub type AllEventHandlers = (SendMobDespawn, SendMobDespawnCoords);
pub type KnownEventSources = (
	systems::missile::MissileCull,
	systems::missile::MissileHit,
	systems::collision::MissileExplodeSystem,
);

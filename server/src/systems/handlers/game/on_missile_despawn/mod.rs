mod send_mob_despawn;
mod send_mob_despawn_coords;

pub use self::send_mob_despawn::SendMobDespawn;
pub use self::send_mob_despawn_coords::SendMobDespawnCoords;

pub type AllEventHandlers = (SendMobDespawn, SendMobDespawnCoords);
pub type KnownEventSources = (
	crate::systems::missile::MissileCull,
	crate::systems::missile::MissileHit,
	crate::systems::collision::MissileExplodeSystem,
);

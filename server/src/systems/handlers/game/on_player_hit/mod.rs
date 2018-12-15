mod inflict_damage;
mod send_packet;

pub use self::inflict_damage::InflictDamage;
pub use self::send_packet::SendPacket;

use systems;

pub type AllPlayerHitSystems = (InflictDamage, SendPacket);
pub type KnownEventSources = (systems::missile::MissileHit);

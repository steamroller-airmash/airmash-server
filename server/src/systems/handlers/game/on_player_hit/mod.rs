mod create_despawn_event;
mod inflict_damage;
mod send_packet;

pub use self::create_despawn_event::CreateDespawnEvent;
pub use self::inflict_damage::InflictDamage;
pub use self::send_packet::SendPacket;

use systems;

pub type AllPlayerHitSystems = (InflictDamage, SendPacket, CreateDespawnEvent);
pub type KnownEventSources = (systems::missile::MissileHit);

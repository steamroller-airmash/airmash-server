mod inflict_damage;
mod send_packet;

pub use self::inflict_damage::InflictDamage;
pub use self::send_packet::SendPacket;

pub type AllPlayerHitSystems = (InflictDamage, SendPacket);

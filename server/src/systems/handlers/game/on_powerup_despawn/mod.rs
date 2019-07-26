mod send_packet;

pub use self::send_packet::SendPacket;

pub type AllPowerupDespawnHandlers = (SendPacket);
pub type KnownEventSources = (crate::systems::upgrades::Despawn);

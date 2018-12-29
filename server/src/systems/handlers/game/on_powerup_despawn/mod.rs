mod send_packet;

pub use self::send_packet::SendPacket;

use systems;

pub type AllPowerupDespawnHandlers = (SendPacket);
pub type KnownEventSources = (systems::upgrades::Despawn);

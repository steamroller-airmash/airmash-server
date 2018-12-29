mod send_packet;

pub use self::send_packet::SendPacket;

use systems;

pub type AllPowerupSpawnHandlers = (SendPacket);

pub type KnownEventSources = (systems::upgrades::SpawnShield);

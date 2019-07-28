mod send_packet;

pub use self::send_packet::SendPacket;

pub type AllPowerupSpawnHandlers = (SendPacket);

pub type KnownEventSources = (
    crate::systems::powerups::SpawnRandomPowerup,
    crate::systems::powerups::SpawnFixedPowerup,
);

mod register;

mod despawn;
mod pickup_upgrade;
mod spawn_shield;

pub use self::register::register;

pub use self::despawn::Despawn;
pub use self::pickup_upgrade::PickupUpgrade as Pickup;
pub use self::spawn_shield::SpawnShield;

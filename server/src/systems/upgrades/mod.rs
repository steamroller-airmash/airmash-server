mod register;

mod despawn;
mod pickup_upgrade;

pub use self::register::register;

pub use self::despawn::Despawn;
pub use self::pickup_upgrade::PickupUpgrade as Pickup;

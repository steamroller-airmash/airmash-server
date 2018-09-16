mod register;

mod give_powerup;
mod spawn_upgrade;
mod teleport;

pub use self::register::register;

pub use self::give_powerup::GivePowerup;
pub use self::spawn_upgrade::SpawnUpgrade;
pub use self::teleport::Teleport;

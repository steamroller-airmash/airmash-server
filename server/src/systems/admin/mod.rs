mod register;

mod crash;
mod debug_print;
mod give_powerup;
mod teleport;

pub use self::register::register;

pub use self::crash::Crash;
pub use self::debug_print::DebugPrint;
pub use self::give_powerup::GivePowerup;
pub use self::teleport::Teleport;

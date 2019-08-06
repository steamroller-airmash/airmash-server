mod register;

mod crash;
mod give_powerup;
mod teleport;
mod debug_print;

pub use self::register::register;

pub use self::crash::Crash;
pub use self::give_powerup::GivePowerup;
pub use self::teleport::Teleport;
pub use self::debug_print::DebugPrint;

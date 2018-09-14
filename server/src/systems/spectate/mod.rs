mod register;

#[deprecated]
mod command;
mod respawn;

pub use self::register::register;

pub use self::command::CommandHandler;

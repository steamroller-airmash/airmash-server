mod spawn_random_powerup;

pub mod on_collision;
pub mod on_despawn;
pub mod on_expire;
pub mod on_spawn;
pub mod upkeep;

mod register;

pub use self::register::register;

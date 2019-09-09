//! All systems used within the main airmash engine.

pub(crate) use self::core::task_timer;

mod disconnect;
mod energy_regen;
mod health_regen;
mod position_update;
mod register;
mod run_futures;

pub mod admin;
pub mod collision;
pub mod core;
pub mod handlers;
pub mod limiting;
pub mod missile;
pub mod notify;
pub mod powerups;
pub mod specials;
pub mod stats;
pub mod timers;
pub mod upgrades;
pub mod visibility;

pub use self::disconnect::Disconnect;
pub use self::energy_regen::EnergyRegenSystem;
pub use self::health_regen::HealthRegenSystem;
pub use self::position_update::PositionUpdate;

pub use self::register::register;

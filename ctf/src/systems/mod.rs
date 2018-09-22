mod drop;
mod drop_on_despawn;
mod drop_on_stealth;
mod flagspeed;
mod pickupflag;
mod pos_update;
mod register;
mod score_detailed;

pub mod flag_event;
pub mod on_flag;
pub mod on_game_start;
pub mod on_game_win;
pub mod on_join;
pub mod on_leave;
pub mod timer;

pub use self::register::register;

pub use self::drop::DropSystem;
pub use self::drop_on_despawn::DropOnDespawn;
pub use self::drop_on_stealth::DropOnStealth;
pub use self::flagspeed::FlagSpeedSystem;
pub use self::pickupflag::PickupFlagSystem;
pub use self::pos_update::PosUpdateSystem;
pub use self::score_detailed::ScoreDetailed;

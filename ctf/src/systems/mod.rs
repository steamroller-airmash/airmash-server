mod drop;
mod drop_on_death;
mod drop_on_spec;
mod flagspeed;
mod handle_leave;
mod leaveupdate;
mod loginupdate;
mod pickupflag;
mod pos_update;
mod register;

pub mod on_flag;
pub mod on_join;

pub use self::register::register;

pub use self::drop::DropSystem;
pub use self::drop_on_death::DropOnDeath;
pub use self::drop_on_spec::DropOnSpec;
pub use self::flagspeed::FlagSpeedSystem;
pub use self::handle_leave::UpdateGameModeOnPlayerLeave;
pub use self::leaveupdate::LeaveUpdateSystem;
pub use self::loginupdate::LoginUpdateSystem;
pub use self::pickupflag::PickupFlagSystem;
pub use self::pos_update::PosUpdateSystem;

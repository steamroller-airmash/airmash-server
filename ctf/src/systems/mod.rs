mod config;
mod drop;
mod flag_message;
mod flagspeed;
mod leaveupdate;
mod loginupdate;
mod pickupflag;
mod pos_update;
mod register;
mod return_flag;
mod sendmessage;

pub use self::register::register;

pub use self::drop::DropSystem;
pub use self::flag_message::PickupMessageSystem;
pub use self::flagspeed::FlagSpeedSystem;
pub use self::leaveupdate::LeaveUpdateSystem;
pub use self::loginupdate::LoginUpdateSystem;
pub use self::pickupflag::PickupFlagSystem;
pub use self::pos_update::PosUpdateSystem;
pub use self::return_flag::ReturnFlagSystem;
pub use self::sendmessage::SendFlagMessageSystem;

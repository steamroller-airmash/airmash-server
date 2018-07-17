
mod flag_message;
mod return_flag;
mod sendmessage;
mod update_score;

pub use self::flag_message::PickupMessageSystem as PickupMessage;
pub use self::return_flag::ReturnFlagSystem as ReturnFlag;
pub use self::sendmessage::SendFlagMessageSystem as SendFlagMessage;
pub use self::update_score::UpdateScore;

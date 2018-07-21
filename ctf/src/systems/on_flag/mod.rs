
mod flag_message;
mod capture_flag;
mod sendmessage;
mod update_score;
mod update_captures;

pub use self::flag_message::PickupMessageSystem as PickupMessage;
pub use self::capture_flag::CaptureFlag;
pub use self::sendmessage::SendFlagMessageSystem as SendFlagMessage;
pub use self::update_score::UpdateScore;
pub use self::update_captures::UpdateCaptures;

mod capture_flag;
mod flag_message;
mod sendmessage;
mod update_captures;
mod update_score;

pub use self::capture_flag::CaptureFlag;
pub use self::flag_message::PickupMessageSystem as PickupMessage;
pub use self::sendmessage::SendFlagMessageSystem as SendFlagMessage;
pub use self::update_captures::UpdateCaptures;
pub use self::update_score::UpdateScore;

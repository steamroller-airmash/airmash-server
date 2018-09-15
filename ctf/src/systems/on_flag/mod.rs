mod check_win;
mod display_banner;
mod send_flag_message;
mod update_captures;
mod update_lastdrop;
mod update_score;

pub use self::check_win::CheckWin;
pub use self::display_banner::PickupMessageSystem as PickupMessage;
pub use self::send_flag_message::SendFlagMessageSystem as SendFlagMessage;
pub use self::update_captures::UpdateCaptures;
pub use self::update_lastdrop::UpdateLastDrop;
pub use self::update_score::UpdateScore;

pub type AllFlagSystems = (
	CheckWin,
	PickupMessage,
	SendFlagMessage,
	UpdateCaptures,
	UpdateLastDrop,
	UpdateScore,
);

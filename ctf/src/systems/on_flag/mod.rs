mod check_win;
mod display_banner;
mod do_return;
mod force_update;
mod send_flag_message;
mod update_captures;
mod update_lastdrop;
mod update_score;

pub use self::check_win::CheckWin;
pub use self::display_banner::PickupMessageSystem as PickupMessage;
pub use self::do_return::DoReturn;
pub use self::force_update::ForceUpdate;
pub use self::send_flag_message::SendFlagMessageSystem as SendFlagMessage;
pub use self::update_captures::UpdateCaptures;
pub use self::update_lastdrop::UpdateLastDrop;
pub use self::update_score::UpdateScore;

#[allow(dead_code)]
pub type AllFlagSystems = (
	CheckWin,
	PickupMessage,
	SendFlagMessage,
	UpdateCaptures,
	UpdateLastDrop,
	UpdateScore,
);

use crate::systems;

pub type KnownEventSources = (
	systems::flag_event::ReturnFlag,
	systems::flag_event::CaptureFlag,
	// don't set this, it causes a dependency loop
	//systems::on_game_win::ResetFlags,
	systems::PickupFlag,
	systems::DropOnDespawn,
	systems::DropOnStealth,
	systems::DropSystem,
	systems::on_respawn::DropFlag,
);

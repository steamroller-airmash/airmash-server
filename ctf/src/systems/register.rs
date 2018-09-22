use component::{FlagCarrier, Flags, IsFlag, LastDrop};

use server::{Builder, Position, Team};
use specs::Builder as SpecsBuilder;
use specs::*;

use std::time::Instant;

use super::*;
use config;

pub fn register<'a, 'b>(world: &mut World, disp: Builder<'a, 'b>) -> Builder<'a, 'b> {
	world.register::<Team>();
	world.register::<Position>();
	world.register::<IsFlag>();
	world.register::<FlagCarrier>();
	world.register::<LastDrop>();

	let lastdrop = LastDrop {
		player: None,
		time: Instant::now(),
	};

	let blue = world
		.create_entity()
		.with(config::BLUE_TEAM)
		.with(config::FLAG_HOME_POS[&config::BLUE_TEAM])
		.with(IsFlag {})
		.with(FlagCarrier(None))
		.with(lastdrop)
		.build();
	let red = world
		.create_entity()
		.with(config::RED_TEAM)
		.with(config::FLAG_HOME_POS[&config::RED_TEAM])
		.with(IsFlag {})
		.with(FlagCarrier(None))
		.with(lastdrop)
		.build();

	world.add_resource(Flags { red, blue });

	disp
		.with_handler::<DropOnDespawn>()
		.with_handler::<DropOnStealth>()
		.with::<ScoreDetailed>()
		// On Leave Events
		.with::<on_leave::UpdateGameMode>()
		.with::<on_leave::Drop>()
		// On Join Events
		.with::<on_join::InitCaptures>()
		.with::<on_join::SendFlagPosition>()
		// Needs to happen after SendFlagPosition
		.with::<PickupFlagSystem>()
		.with::<DropSystem>()
		.with::<PosUpdateSystem>()
		.with::<FlagSpeedSystem>()
		// On Flag Events
		.with::<on_flag::SendFlagMessage>()
		.with::<on_flag::PickupMessage>()
		.with::<on_flag::UpdateScore>()
		.with::<on_flag::UpdateCaptures>()
		.with::<on_flag::UpdateLastDrop>()
		.with::<on_flag::CheckWin>()
		// Flag event sending systems
		.with::<flag_event::CaptureFlag>()
		.with::<flag_event::ReturnFlag>()
		// On Game Win events
		.with::<on_game_win::SetupMessages>()
		.with::<on_game_win::SetupGameStart>()
		.with::<on_game_win::SetupReteam>()
		.with::<on_game_win::ChangeConfig>()
		.with::<on_game_win::DisplayWin>()
		.with::<on_game_win::SetGameActive>()
		.with::<on_game_win::AwardBounty>()
		.with_handler::<on_game_win::ResetFlags>()
		// Timer events
		.with::<timer::RestoreConfig>()
		.with::<timer::GameStart>()
		.with::<timer::SetGameActive>()
		.with::<timer::Shuffle>()
		// Game Start events
		.with::<on_game_start::RespawnAllUnspec>()
		.with::<on_game_start::RespawnAll>()
		.with::<on_game_start::ResetScore>()
}

use crate::component::{FlagCarrier, Flags, IsFlag, LastDrop};

use crate::server::protocol::MobType;
use crate::server::{Builder, Position, Team};
use specs::Builder as SpecsBuilder;
use specs::*;

use std::time::{Duration, Instant};

use super::*;
use crate::server::types::*;
use crate::config;

pub fn register<'a, 'b>(world: &mut World, disp: Builder<'a, 'b>) -> Builder<'a, 'b> {
	// Normally the systems would take care of this,
	// but these need to be registered in order to
	// create the flag entities.
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

	world.add_resource(PowerupSpawnPoints(vec![
		PowerupSpawnPoint {
			pos: Position::new(Distance::new(-9300.0), Distance::new(-1470.0)),
			powerup_type: MobType::Shield,
			respawn_delay: Duration::from_secs(105),
			next_respawn_time: None,
			powerup_entity: None,
		},
		PowerupSpawnPoint {
			pos: Position::new(Distance::new(-7440.0), Distance::new(-1380.0)),
			powerup_type: MobType::Inferno,
			respawn_delay: Duration::from_secs(105),
			next_respawn_time: None,
			powerup_entity: None,
		},
		PowerupSpawnPoint {
			pos: Position::new(Distance::new(6550.0), Distance::new(-940.0)),
			powerup_type: MobType::Inferno,
			respawn_delay: Duration::from_secs(105),
			next_respawn_time: None,
			powerup_entity: None,
		},
		PowerupSpawnPoint {
			pos: Position::new(Distance::new(8350.0), Distance::new(-940.0)),
			powerup_type: MobType::Shield,
			respawn_delay: Duration::from_secs(105),
			next_respawn_time: None,
			powerup_entity: None,
		},
		PowerupSpawnPoint {
			pos: Position::new(Distance::new(920.0), Distance::new(-2800.0)),
			powerup_type: MobType::Inferno,
			respawn_delay: Duration::from_secs(60),
			next_respawn_time: None,
			powerup_entity: None,
		},
	]));

	disp.with_handler::<DropOnDespawn>()
		.with_handler::<DropOnStealth>()
		.with_handler::<ScoreDetailed>()
		// On Leave Events
		.with_handler::<on_leave::UpdateGameMode>()
		.with_handler::<on_leave::Drop>()
		// On Join Events
		.with_handler::<on_join::InitCaptures>()
		.with_handler::<on_join::SendFlagPosition>()
		// Needs to happen after SendFlagPosition
		.with::<PickupFlagSystem>()
		.with::<PosUpdateSystem>()
		.with_handler::<DropSystem>()
		.with_handler::<FlagSpeedSystem>()
		// On Flag Events
		.with_handler::<on_flag::SendFlagMessage>()
		.with_handler::<on_flag::PickupMessage>()
		.with_handler::<on_flag::UpdateScore>()
		.with_handler::<on_flag::UpdateCaptures>()
		.with_handler::<on_flag::UpdateLastDrop>()
		.with_handler::<on_flag::CheckWin>()
		.with_handler::<on_flag::DoReturn>()
		.with_handler::<on_flag::ForceUpdate>()
		// Flag event sending systems
		.with::<flag_event::CaptureFlag>()
		.with::<flag_event::ReturnFlag>()
		// On Game Win events
		.with_handler::<on_game_win::SetupMessages>()
		.with_handler::<on_game_win::SetupGameStart>()
		.with_handler::<on_game_win::SetupReteam>()
		.with_handler::<on_game_win::ChangeConfig>()
		.with_handler::<on_game_win::DisplayWin>()
		.with_handler::<on_game_win::SetGameActive>()
		.with_handler::<on_game_win::AwardBounty>()
		.with_handler::<on_game_win::ResetFlags>()
		// Timer events
		.with_handler::<timer::RestoreConfig>()
		.with_handler::<timer::GameStart>()
		.with_handler::<timer::SetGameActive>()
		.with_handler::<timer::Shuffle>()
		// Game Start events
		.with_handler::<on_game_start::RespawnAllUnspec>()
		.with_handler::<on_game_start::RespawnAll>()
		.with_handler::<on_game_start::ResetScore>()
}

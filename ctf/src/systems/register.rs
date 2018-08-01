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
		.with::<PickupFlagSystem>()
		.with::<DropSystem>()
		.with::<PosUpdateSystem>()
		.with::<FlagSpeedSystem>()
		.with::<DropOnSpec>()
		.with::<DropOnDeath>()
		// On Leave Events
		.with::<on_leave::UpdateGameMode>()
		.with::<on_leave::Drop>()
		// On Join Events
		.with::<on_join::InitCaptures>()
		.with::<on_join::SendFlagPosition>()
		// On Flag Events
		.with::<on_flag::SendFlagMessage>()
		.with::<on_flag::PickupMessage>()
		.with::<on_flag::UpdateScore>()
		.with::<on_flag::UpdateCaptures>()
		// Flag event sending systems
		.with::<flag_event::CaptureFlag>()
		.with::<flag_event::ReturnFlag>()
		// On Game Win events
		.with::<on_game_win::SetupMessages>()
}

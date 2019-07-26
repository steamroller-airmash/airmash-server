use specs::*;

use crate::component::event::*;
use crate::component::flag::IsDead;
use crate::types::*;
use crate::SystemInfo;

use crate::utils::{EventHandler, EventHandlerTypeProvider};

use crate::systems::handlers::command::AllCommandHandlers;
use crate::systems::handlers::game::on_join::AllJoinHandlers;

/// Set transform, health, energy and flags
/// for a player when they respawn.
///
/// More specifically, this system sets the following:
///
/// - [`Position`]
/// - [`Velocity`]
/// - [`Rotation`]
/// - [`Health`]
/// - [`Energy`]
#[derive(Default)]
pub struct SetTraits;

#[derive(SystemData)]
pub struct SetTraitsData<'a> {
	entities: Entities<'a>,
	team: ReadStorage<'a, Team>,
	pos: WriteStorage<'a, Position>,
	vel: WriteStorage<'a, Velocity>,
	rot: WriteStorage<'a, Rotation>,
	health: WriteStorage<'a, Health>,
	energy: WriteStorage<'a, Energy>,
	is_dead: WriteStorage<'a, IsDead>,

	gamemode: GameModeWriter<'a, dyn GameMode>,
}

impl EventHandlerTypeProvider for SetTraits {
	type Event = PlayerRespawn;
}

impl<'a> EventHandler<'a> for SetTraits {
	type SystemData = SetTraitsData<'a>;

	fn on_event(&mut self, evt: &PlayerRespawn, data: &mut Self::SystemData) {
		if !data.entities.is_alive(evt.player) {
			return;
		}

		let gamemode = data.gamemode.get_mut();

		let player = evt.player;
		let team = *try_get!(player, data.team);
		let pos = gamemode.spawn_pos(player, team);

		data.is_dead.remove(player);
		data.pos.insert(player, pos).unwrap();
		data.vel.insert(player, Velocity::default()).unwrap();
		data.rot.insert(player, Rotation::default()).unwrap();
		data.health.insert(player, Health::new(1.0)).unwrap();
		data.energy.insert(player, Energy::new(1.0)).unwrap();
	}
}

impl SystemInfo for SetTraits {
	type Dependencies = (
		AllJoinHandlers,
		AllCommandHandlers,
		super::CreateDespawnEvent,
	);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

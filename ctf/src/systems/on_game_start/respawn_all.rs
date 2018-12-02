use specs::*;

use server::component::channel::*;
use server::component::event::*;
use server::component::flag::*;
use server::systems::handlers::game::on_join::AllJoinHandlers;
use server::types::systemdata::IsAlive;
use server::utils::*;
use server::*;

use systems::timer::GameStart;

use super::RespawnAllUnspec;

/// Respawn all players at the start of
/// the game.
#[derive(Default)]
pub struct RespawnAll;

#[derive(SystemData)]
pub struct RespawnAllData<'a> {
	respawn_channel: Write<'a, OnPlayerRespawn>,

	entities: Entities<'a>,
	is_player: ReadStorage<'a, IsPlayer>,
	is_alive: IsAlive<'a>,
}

impl EventHandlerTypeProvider for RespawnAll {
	type Event = GameStart;
}

impl<'a> EventHandler<'a> for RespawnAll {
	type SystemData = RespawnAllData<'a>;

	fn on_event(&mut self, _: &GameStart, data: &mut Self::SystemData) {
		use self::PlayerRespawnPrevStatus::*;

		let players = (&*data.entities, data.is_player.mask())
			.join()
			.map(|(ent, ..)| (ent, data.is_alive.get(ent)))
			.map(|(ent, is_alive)| PlayerRespawn {
				player: ent,
				prev_status: if is_alive { Alive } else { Dead },
			})
			.collect::<Vec<_>>();

		data.respawn_channel.iter_write(players.into_iter());
	}
}

impl SystemInfo for RespawnAll {
	type Dependencies = (
		// We need to depend on RespawnAllUnspec
		// since PlayerRespawner will only
		// send out the PlayerRespawn packet
		// if the player is not also spectating.
		// While in most cases that is the desired
		// behaviour, it doesn't work when we're
		// planning to drop all players out of spec
		// too unless they're dropped out of spec
		// first, which is why this dependency is here.
		RespawnAllUnspec,
		// PlayerRespawn accesses position by entity,
		// there's a race condition in ordering here
		// if a player joins exactly as a game is starting.
		AllJoinHandlers,
		// We want to run in the same frame as the
		// GameStart event is triggered.
		GameStart,
	);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

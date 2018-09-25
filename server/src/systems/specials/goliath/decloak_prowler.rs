use specs::*;
use types::systemdata::*;
use types::*;

use SystemInfo;

use component::channel::{OnPlayerRepel, OnPlayerRepelReader};
use component::flag::IsPlayer;
use component::time::{LastStealthTime, ThisFrame};
use systems::specials::config::*;

/// Send [`EventRepel`] when a goliath uses it's special.
///
/// This system also position, speed, velocity,
/// team and owner for players and mobs that
/// are caught in the deflection.
#[derive(Default)]
pub struct DecloakProwler {
	reader: Option<OnPlayerRepelReader>,
}

#[derive(SystemData)]
pub struct DecloakProwlerData<'a> {
	channel: Read<'a, OnPlayerRepel>,
	entities: Entities<'a>,
	this_frame: Read<'a, ThisFrame>,

	pos: ReadStorage<'a, Position>,
	team: WriteStorage<'a, Team>,
	keystate: WriteStorage<'a, KeyState>,
	last_stealth: WriteStorage<'a, LastStealthTime>,
	is_alive: IsAlive<'a>,
	is_player: ReadStorage<'a, IsPlayer>,
}

impl DecloakProwler {}

impl<'a> System<'a> for DecloakProwler {
	type SystemData = DecloakProwlerData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerRepel>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		let player_r2 = *GOLIATH_SPECIAL_RADIUS_PLAYER * *GOLIATH_SPECIAL_RADIUS_PLAYER;

		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let pos = *data.pos.get(evt.player).unwrap();
			let team = *data.team.get(evt.player).unwrap();

			let hit_players = (
				&*data.entities,
				&data.pos,
				&data.team,
				data.is_player.mask(),
				data.is_alive.mask(),
			)
				.join()
				.filter(|(ent, ..)| *ent != evt.player)
				.filter(|(_, _, player_team, ..)| **player_team != team)
				.filter_map(|(ent, player_pos, ..)| {
					let dist2 = (*player_pos - pos).length2();

					if dist2 < player_r2 {
						Some(ent)
					} else {
						None
					}
				}).collect::<Vec<_>>();

			for player in hit_players {
				let keystate = data.keystate.get_mut(player).unwrap();

				keystate.stealthed = false;
				keystate.special = false;

				data.last_stealth
					.insert(player, LastStealthTime(data.this_frame.0))
					.unwrap();
			}
		}
	}
}

impl SystemInfo for DecloakProwler {
	type Dependencies = super::GoliathRepel;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

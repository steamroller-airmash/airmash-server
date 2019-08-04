use crate::server::*;
use specs::*;

use crate::config as ctfconfig;

use crate::component::*;

use std::cmp::Ordering;

use crate::server::component::flag::IsPlayer;
use crate::server::types::systemdata::*;

#[derive(Default)]
pub struct ReturnFlag;

#[derive(SystemData)]
pub struct ReturnFlagData<'a> {
	ents: Entities<'a>,
	pos: WriteStorage<'a, Position>,
	team: ReadStorage<'a, Team>,
	plane: ReadStorage<'a, Plane>,
	is_flag: ReadStorage<'a, IsFlag>,
	is_player: ReadStorage<'a, IsPlayer>,
	carrier: ReadStorage<'a, FlagCarrier>,
	keystate: ReadStorage<'a, KeyState>,
	is_alive: IsAlive<'a>,

	channel: Write<'a, OnFlag>,
}

impl<'a> System<'a> for ReturnFlag {
	type SystemData = ReturnFlagData<'a>;

	fn run(&mut self, data: Self::SystemData) {
		let ents = data.ents;
		let pos = data.pos;
		let team = data.team;
		let plane = data.plane;
		let is_flag = data.is_flag;
		let is_player = data.is_player;
		let carrier = data.carrier;
		let keystate = data.keystate;
		let is_alive = data.is_alive;
		let mut channel = data.channel;

		let returned = {
			let flags = {
				(&*ents, &pos, &team, &carrier, &is_flag)
					.join()
					.filter(|(_, _, _, carrier, ..)| carrier.0.is_none())
					.filter(|(_, pos, team, ..)| {
						(ctfconfig::FLAG_HOME_POS[&team] - **pos).length2().inner() > 0.01
					})
					.map(|(ent, pos, team, ..)| (ent, *pos, *team))
					.collect::<Vec<_>>()
			};

			if flags.len() == 0 {
				return;
			}

			flags
				.iter()
				.filter_map(|(flag, flag_pos, flag_team)| {
					let mut possible_returns = (
						&*ents,
						&pos,
						&plane,
						&team,
						&keystate,
						&is_player,
						is_alive.mask(),
					)
						.join()
						.filter(|(_, _, _, player_team, ..)| **player_team == *flag_team)
						.filter(|(_, _, _, _, keystate, ..)| !keystate.stealthed)
						.filter_map(|(player, player_pos, plane, ..)| {
							let radius = ctfconfig::FLAG_RADIUS[&plane];
							let dist2 = (*player_pos - *flag_pos).length2();

							if dist2 < radius * radius {
								Some((player, dist2))
							} else {
								None
							}
						})
						.collect::<Vec<_>>();

					possible_returns
						.sort_by(|(_, a), (_, b)| a.partial_cmp(&b).unwrap_or(Ordering::Greater));

					possible_returns.first().map(|(player, _)| (*player, *flag))
				})
				.collect::<Vec<_>>()
		};

		for (player, flag) in returned {
			channel.single_write(FlagEvent {
				ty: FlagEventType::Return,
				player: Some(player),
				flag: flag,
			});
		}
	}
}

system_info! {
	impl SystemInfo for ReturnFlag {
		type Dependencies = crate::systems::PickupFlag;
	}
}

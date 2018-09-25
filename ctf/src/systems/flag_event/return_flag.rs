use server::*;
use specs::*;

use config as ctfconfig;

use component::*;

use std::cmp::Ordering;

use server::component::flag::IsPlayer;
use server::protocol::server::GameFlag;
use server::protocol::FlagUpdateType;
use server::types::systemdata::*;

pub struct ReturnFlag;

#[derive(SystemData)]
pub struct ReturnFlagData<'a> {
	pub ents: Entities<'a>,
	pub pos: WriteStorage<'a, Position>,
	pub team: ReadStorage<'a, Team>,
	pub plane: ReadStorage<'a, Plane>,
	pub is_flag: ReadStorage<'a, IsFlag>,
	pub is_player: ReadStorage<'a, IsPlayer>,
	pub carrier: ReadStorage<'a, FlagCarrier>,
	pub is_alive: IsAlive<'a>,

	pub scores: Read<'a, GameScores>,
	pub channel: Write<'a, OnFlag>,
	pub conns: Read<'a, Connections>,
}

impl<'a> System<'a> for ReturnFlag {
	type SystemData = ReturnFlagData<'a>;

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			ents,
			mut pos,
			team,
			plane,
			is_flag,
			is_player,
			carrier,
			is_alive,

			scores,
			mut channel,
			conns,
		} = data;

		let returned = {
			let flags = {
				(&*ents, &pos, &team, &carrier, &is_flag)
					.join()
					.filter(|(_, _, _, carrier, ..)| carrier.0.is_none())
					.filter(|(_, pos, team, ..)| {
						(ctfconfig::FLAG_HOME_POS[&team] - **pos).length2().inner() > 0.01
					}).map(|(ent, pos, team, ..)| (ent, *pos, *team))
					.collect::<Vec<_>>()
			};

			if flags.len() == 0 {
				return;
			}

			flags
				.iter()
				.filter_map(|(flag, flag_pos, flag_team)| {
					let mut possible_returns =
						(&*ents, &pos, &plane, &team, &is_player, is_alive.mask())
							.join()
							.filter(|(_, _, _, player_team, ..)| **player_team == *flag_team)
							.filter_map(|(player, player_pos, plane, ..)| {
								let radius = ctfconfig::FLAG_RADIUS[&plane];
								let dist2 = (*player_pos - *flag_pos).length2();

								if dist2 < radius * radius {
									Some((player, dist2))
								} else {
									None
								}
							}).collect::<Vec<_>>();

					possible_returns
						.sort_by(|(_, a), (_, b)| a.partial_cmp(&b).unwrap_or(Ordering::Greater));

					possible_returns
						.first()
						.map(|(player, _)| (*player, *flag, *flag_team))
				}).collect::<Vec<_>>()
		};

		for (player, flag, team) in returned {
			let flag_pos = pos.get_mut(flag).unwrap();

			info!("{:?}", flag_pos);

			*flag_pos = ctfconfig::FLAG_HOME_POS[&team];

			let packet = GameFlag {
				ty: FlagUpdateType::Position,
				flag: Flag(team),
				id: None,
				pos: *flag_pos,
				blueteam: scores.blueteam,
				redteam: scores.redteam,
			};

			conns.send_to_all(packet);

			channel.single_write(FlagEvent {
				ty: FlagEventType::Return,
				player: Some(player),
				flag: flag,
			});
		}
	}
}

use systems::PickupFlagSystem;

impl SystemInfo for ReturnFlag {
	type Dependencies = PickupFlagSystem;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self {}
	}
}

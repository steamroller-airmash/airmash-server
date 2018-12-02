use specs::*;

use server::component::flag::*;
use server::component::time::ThisFrame;
use server::types::systemdata::*;
use server::*;

use component::*;
use config as ctfconfig;
use systems::on_join::SendFlagPosition;

use std::cmp::Ordering;

pub struct PickupFlagSystem;

#[derive(SystemData)]
pub struct PickupFlagSystemData<'a> {
	entities: Entities<'a>,
	channel: Write<'a, OnFlag>,
	thisframe: Read<'a, ThisFrame>,
	game_active: Read<'a, GameActive>,

	// Player data
	plane: ReadStorage<'a, Plane>,
	is_player: ReadStorage<'a, IsPlayer>,
	is_alive: IsAlive<'a>,

	// These ones are for both
	pos: WriteStorage<'a, Position>,
	team: ReadStorage<'a, Team>,

	// Flag Data
	is_flag: ReadStorage<'a, IsFlag>,
	carrier: WriteStorage<'a, FlagCarrier>,
	lastdrop: ReadStorage<'a, LastDrop>,

	keystate: ReadStorage<'a, KeyState>,
}

impl<'a> System<'a> for PickupFlagSystem {
	type SystemData = PickupFlagSystemData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		// Flags can only be captured when a game is active
		if !data.game_active.0 {
			return;
		}

		let flags = (
			&*data.entities,
			&data.pos,
			&data.team,
			&data.carrier,
			&data.is_flag,
			&data.lastdrop,
		)
			.join()
			.map(|(ent, pos, team, carrier, _, lastdrop)| (ent, *pos, *team, *carrier, *lastdrop))
			.collect::<Vec<(Entity, Position, Team, FlagCarrier, LastDrop)>>();

		for (f_ent, f_pos, f_team, carrier, lastdrop) in flags {
			if carrier.0.is_some() {
				continue;
			}

			let nearest = (
				&*data.entities,
				&data.pos,
				&data.team,
				&data.is_player,
				&data.plane,
				data.is_alive.mask(),
				&data.keystate,
			)
				.join()
				.filter(|(_, _, p_team, ..)| f_team != **p_team)
				.filter(|(ent, ..)| {
					// Check against time-since-drop
					(data.thisframe.0 - lastdrop.time) > *ctfconfig::FLAG_NO_REGRAB_TIME
						// Then check against contained player id
						|| lastdrop.player.map(|x| x != *ent).unwrap_or(false)
				})
				.filter(|(_, _, _, _, _, _, ref keystate)| keystate.stealthed != true)
				.filter_map(|(p_ent, p_pos, _, _, p_plane, ..)| {
					let rad = ctfconfig::FLAG_RADIUS[&p_plane];
					let dst = (*p_pos - f_pos).length2();

					// Filter out distances that are too large
					// to pick up the flag
					if dst > rad * rad {
						None
					} else {
						// Comparing squared distances has the same
						// properties as using the actual distance
						Some((p_ent, dst - rad * rad))
					}
				})
				.min_by(|a, b| {
					if a.1 < b.1 {
						Ordering::Less
					} else {
						Ordering::Greater
					}
				});

			if nearest.is_none() {
				continue;
			}

			let nearest = nearest.unwrap().0;
			let team = *match log_none!(nearest, data.team) {
				Some(x) => x,
				None => continue,
			};

			data.carrier
				.insert(f_ent, FlagCarrier(Some(nearest)))
				.unwrap();

			let ty = if team == f_team {
				FlagEventType::Return
			} else {
				FlagEventType::PickUp
			};

			data.channel.single_write(FlagEvent {
				ty,
				player: Some(nearest),
				flag: f_ent,
			});
		}
	}
}

use server::systems::PositionUpdate;

impl SystemInfo for PickupFlagSystem {
	type Dependencies = (PositionUpdate, SendFlagPosition);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self {}
	}
}

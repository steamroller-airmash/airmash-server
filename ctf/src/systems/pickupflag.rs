use specs::*;

use server::component::flag::*;
use server::component::time::ThisFrame;
use server::types::systemdata::*;
use server::types::Sqrt;
use server::*;

use component::*;
use config as ctfconfig;
use systems::on_join::SendFlagPosition;

use std::cmp::Ordering;

pub struct PickupFlagSystem;

#[derive(SystemData)]
pub struct PickupFlagSystemData<'a> {
	pub config: Read<'a, Config>,
	pub entities: Entities<'a>,
	pub channel: Write<'a, OnFlag>,
	pub thisframe: Read<'a, ThisFrame>,
	pub game_active: Read<'a, GameActive>,

	// Player data
	pub plane: ReadStorage<'a, Plane>,
	pub is_player: ReadStorage<'a, IsPlayer>,
	pub is_alive: IsAlive<'a>,

	// These ones are for both
	pub pos: WriteStorage<'a, Position>,
	pub team: ReadStorage<'a, Team>,

	// Flag Data
	pub is_flag: ReadStorage<'a, IsFlag>,
	pub carrier: WriteStorage<'a, FlagCarrier>,
	pub lastdrop: ReadStorage<'a, LastDrop>,

	pub keystate: ReadStorage<'a, KeyState>,
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
				}).filter(|(_, _, _, _, _, _, ref keystate)| keystate.stealthed != true)
				.filter_map(|(p_ent, p_pos, _, _, p_plane, ..)| {
					let rad = ctfconfig::FLAG_RADIUS[&p_plane];
					let dst = (*p_pos - f_pos).length2();

					// Quickly filter out negative distances
					// without doing a sqrt
					if dst > rad * rad {
						None
					} else {
						// Only calculate actual distance if necessary
						Some((p_ent, dst.sqrt() - rad))
					}
				}).min_by(|a, b| {
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
			let team = *data.team.get(nearest).unwrap();

			*data.carrier.get_mut(f_ent).unwrap() = FlagCarrier(Some(nearest));

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

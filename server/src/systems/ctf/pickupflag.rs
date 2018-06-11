
use specs::*;
use types::*;
use dimensioned::Sqrt;

use systems::ctf::config as ctfconfig;
use component::ctf::*;
use component::flag::IsPlayer;

use std::cmp::Ordering;

pub struct PickupFlagSystem;

#[derive(SystemData)]
pub struct PickupFlagSystemData<'a> {
	pub config:   Read<'a, Config>,
	pub entities: Entities<'a>,
	pub channel:  Write<'a, OnFlag>,

	// Player data
	pub plane: ReadStorage<'a, Plane>,
	pub is_player: ReadStorage<'a, IsPlayer>,

	// These ones are for both
	pub pos:  ReadStorage<'a, Position>,
	pub team: ReadStorage<'a, Team>,

	// Flag Data
	pub is_flag: ReadStorage<'a, IsFlag>,
	pub carrier: WriteStorage<'a, FlagCarrier>,
}

impl<'a> System<'a> for PickupFlagSystem {
	type SystemData = PickupFlagSystemData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let flags = (
			&*data.entities,
			&data.pos,
			&data.team,
			&data.carrier,
			&data.is_flag
		).join()
			.map(|(ent, pos, team, carrier, _)| {
				(ent, *pos, *team, *carrier)
			})
			.collect::<Vec<(Entity, Position, Team, FlagCarrier)>>();

		for (f_ent, f_pos, f_team, carrier) in flags {
			if carrier.0.is_some() { continue; }

			let nearest = (
				&*data.entities,
				&data.pos,
				&data.team,
				&data.is_player,
				&data.plane
			).join()
				.filter(|(_, _, p_team, _, _)| f_team != **p_team)
				.filter_map(|(p_ent, p_pos, _, _, p_plane)| {
					let rad = ctfconfig::FLAG_RADIUS[&p_plane];
					let dst = (*p_pos - f_pos).length2();

					// Quickly filter out negative distances
					// without doing a sqrt
					if dst > rad * rad { 
						None
					}
					else {
						// Only calculate actual distance if necessary
						Some((p_ent, dst.sqrt() - rad))						
					}
				})
				.min_by(|a, b| if a.1 < b.1 { Ordering::Less } else { Ordering::Greater });

			if nearest.is_none() {
				continue;
			}

			let nearest = nearest.unwrap().0;

			*data.carrier.get_mut(f_ent).unwrap() = FlagCarrier(Some(nearest));

			data.channel.single_write(FlagEvent {
				ty: FlagEventType::PickUp,
				carrier: Some(nearest),
				flag: f_ent
			});
		}
	}
}

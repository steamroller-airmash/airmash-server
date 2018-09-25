use specs::*;
use types::*;

use component::missile::MissileTrajectory;
use dispatch::SystemInfo;

use airmash_protocol::server::MobDespawn;

pub struct MissileCull;

#[derive(SystemData)]
pub struct MissileCullData<'a> {
	pub ents: Entities<'a>,
	pub missile_trajectory: ReadStorage<'a, MissileTrajectory>,
	pub pos: ReadStorage<'a, Position>,
	pub mob: ReadStorage<'a, Mob>,
	pub conns: Read<'a, Connections>,
}

impl<'a> System<'a> for MissileCull {
	type SystemData = MissileCullData<'a>;

	fn run(&mut self, data: MissileCullData<'a>) {
		(&*data.ents, &data.mob, &data.pos, &data.missile_trajectory)
			.join()
			.filter_map(|(ent, mob, pos, missile_trajectory)| {
				let distance_traveled = (*pos - missile_trajectory.0).length();
				let end_distance = missile_trajectory.1;
				if distance_traveled > end_distance {
					Some((ent, *mob))
				} else {
					None
				}
			}).for_each(|(ent, mob)| {
				data.ents.delete(ent).unwrap();

				let packet = MobDespawn {
					id: ent.into(),
					ty: mob,
				};

				data.conns.send_to_all(packet);
			});
	}
}

impl SystemInfo for MissileCull {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self {}
	}
}

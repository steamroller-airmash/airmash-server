use specs::*;
use types::*;

use component::missile::MissileTrajectory;
use component::reference::PlayerRef;
use component::flag::IsMissile;
use component::event::TimerEvent;
use dispatch::SystemInfo;
use consts::timer::DELETE_ENTITY;
use consts::missile::ID_REUSE_TIME;

use airmash_protocol::server::MobDespawn;

pub struct MissileCull;

#[derive(SystemData)]
pub struct MissileCullData<'a> {
	ents: Entities<'a>,
	missile_trajectory: ReadStorage<'a, MissileTrajectory>,
	pos: ReadStorage<'a, Position>,
	mob: ReadStorage<'a, Mob>,
	conns: Read<'a, Connections>,
	lazy: Read<'a, LazyUpdate>,
	dispatch: ReadExpect<'a, FutureDispatcher>,
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
			})
			.for_each(|(ent, mob)| {

			// Remove a bunch of components that are used to 
			// recognize missiles lazily (they will get
			// removed at the end of the frame)
			data.lazy.remove::<Position>(ent);
			data.lazy.remove::<Mob>(ent);
			data.lazy.remove::<IsMissile>(ent);
			data.lazy.remove::<Velocity>(ent);
			data.lazy.remove::<Team>(ent);
			data.lazy.remove::<PlayerRef>(ent);

			data.dispatch.run_delayed(*ID_REUSE_TIME, move |inst| {
				TimerEvent {
					ty: *DELETE_ENTITY,
					instant: inst,
					data: Some(Box::new(ent))
				}
			});

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

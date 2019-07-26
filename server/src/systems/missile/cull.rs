use specs::*;

use crate::component::channel::OnMissileDespawn;
use crate::component::event::{MissileDespawn, MissileDespawnType, TimerEvent};
use crate::component::flag::IsMissile;
use crate::component::missile::MissileTrajectory;
use crate::component::reference::PlayerRef;
use crate::consts::missile::ID_REUSE_TIME;
use crate::consts::timer::DELETE_ENTITY;
use crate::dispatch::SystemInfo;
use crate::types::*;

pub struct MissileCull;

#[derive(SystemData)]
pub struct MissileCullData<'a> {
	ents: Entities<'a>,
	missile_trajectory: ReadStorage<'a, MissileTrajectory>,
	mob: ReadStorage<'a, Mob>,
	pos: ReadStorage<'a, Position>,
	is_missile: ReadStorage<'a, IsMissile>,
	lazy: Read<'a, LazyUpdate>,
	dispatch: ReadExpect<'a, FutureDispatcher>,
	channel: Write<'a, OnMissileDespawn>,
}

impl<'a> System<'a> for MissileCull {
	type SystemData = MissileCullData<'a>;

	fn run(&mut self, mut data: MissileCullData<'a>) {
		let ref mut channel = data.channel;
		let ref lazy = data.lazy;
		let ref dispatch = data.dispatch;

		(
			&*data.ents,
			&data.pos,
			&data.mob,
			&data.missile_trajectory,
			data.is_missile.mask(),
		)
			.join()
			.filter_map(|(ent, pos, mob, missile_trajectory, ..)| {
				let distance_traveled = (*pos - missile_trajectory.0).length();
				let end_distance = missile_trajectory.1;
				if distance_traveled > end_distance {
					Some((ent, *mob, *pos))
				} else {
					None
				}
			})
			.for_each(|(ent, mob, pos)| {
				// Remove a bunch of components that are used to
				// recognize missiles lazily (they will get
				// removed at the end of the frame)
				lazy.remove::<Position>(ent);
				lazy.remove::<Mob>(ent);
				lazy.remove::<IsMissile>(ent);
				lazy.remove::<Velocity>(ent);
				lazy.remove::<Team>(ent);
				lazy.remove::<PlayerRef>(ent);

				dispatch.run_delayed(ID_REUSE_TIME, move |inst| TimerEvent {
					ty: *DELETE_ENTITY,
					instant: inst,
					data: Some(Box::new(ent)),
				});

				channel.single_write(MissileDespawn {
					ty: MissileDespawnType::LifetimeEnded,
					missile: ent,
					pos,
					mob,
				});
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

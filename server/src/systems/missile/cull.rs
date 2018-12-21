use specs::*;

use types::systemdata::*;
use types::*;

use component::channel::OnMissileDespawn;
use component::event::{MissileDespawn, MissileDespawnType, TimerEvent};
use component::flag::IsMissile;
use component::missile::MissileTrajectory;
use component::reference::PlayerRef;
use consts::missile::ID_REUSE_TIME;
use consts::timer::DELETE_ENTITY;
use dispatch::SystemInfo;

use protocol::server::MobDespawn;

pub struct MissileCull;

#[derive(SystemData)]
pub struct MissileCullData<'a> {
	ents: Entities<'a>,
	missile_trajectory: ReadStorage<'a, MissileTrajectory>,
	pos: ReadStorage<'a, Position>,
	mob: ReadStorage<'a, Mob>,
	conns: SendToVisible<'a>,
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
		let ref conns = data.conns;

		(&*data.ents, &data.mob, &data.pos, &data.missile_trajectory)
			.join()
			.filter_map(|(ent, mob, pos, missile_trajectory)| {
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

				dispatch.run_delayed(*ID_REUSE_TIME, move |inst| TimerEvent {
					ty: *DELETE_ENTITY,
					instant: inst,
					data: Some(Box::new(ent)),
				});

				conns.send_to_visible(
					pos,
					MobDespawn {
						id: ent.into(),
						ty: mob.into(),
					},
				);

				channel.single_write(MissileDespawn {
					missile: ent,
					pos,
					ty: MissileDespawnType::LifetimeEnded,
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

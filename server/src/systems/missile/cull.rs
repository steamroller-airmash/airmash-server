use specs::prelude::*;

use crate::component::channel::OnMissileDespawn;
use crate::component::event::{MissileDespawn, MissileDespawnType};
use crate::component::flag::{IsMissile, IsZombie};
use crate::component::missile::MissileTrajectory;
use crate::component::reference::PlayerRef;
use crate::consts::missile::ID_REUSE_TIME;
use crate::types::*;
use crate::utils::HistoricalStorageExt;

#[derive(Default)]
pub struct MissileCull;

#[derive(SystemDataCustom)]
pub struct MissileCullData<'a> {
	ents: Entities<'a>,
	missile_trajectory: ReadStorage<'a, MissileTrajectory>,
	mob: ReadStorage<'a, Mob>,
	pos: ReadStorage<'a, Position>,
	is_missile: ReadStorage<'a, IsMissile>,
	is_zombie: WriteStorage<'a, IsZombie>,

	lazy: Read<'a, LazyUpdate>,
	tasks: WriteExpect<'a, TaskSpawner>,
	channel: Write<'a, OnMissileDespawn>,
}

impl<'a> System<'a> for MissileCull {
	type SystemData = MissileCullData<'a>;

	fn run(&mut self, mut data: MissileCullData<'a>) {
		let ref mut channel = data.channel;
		let ref lazy = data.lazy;
		let ref mut is_zombie = data.is_zombie;
		let ref tasks = data.tasks;

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
				if is_zombie.mask().contains(ent.id()) {
					return;
				}

				// Remove a bunch of components that are used to
				// recognize missiles lazily (they will get
				// removed at the end of the frame)
				lazy.remove::<Position>(ent);
				lazy.remove::<Mob>(ent);
				lazy.remove::<IsMissile>(ent);
				lazy.remove::<Velocity>(ent);
				lazy.remove::<Team>(ent);
				lazy.remove::<PlayerRef>(ent);

				is_zombie
					.insert_with_history(ent, IsZombie::from_sys(self))
					.unwrap();

				tasks.spawn(crate::task::delayed_delete(
					tasks.task_data(),
					ent,
					ID_REUSE_TIME,
				));

				channel.single_write(MissileDespawn {
					ty: MissileDespawnType::LifetimeEnded,
					missile: ent,
					pos,
					mob,
				});
			});
	}
}

system_info! {
	impl SystemInfo for MissileCull {
		type Dependencies = super::MissileHit;
	}
}

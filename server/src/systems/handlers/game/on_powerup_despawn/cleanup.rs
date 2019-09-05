use crate::{
	component::{
		event::PowerupDespawnEvent,
		flag::{IsPowerup, IsZombie},
	},
	consts::missile::ID_REUSE_TIME,
	types::{Mob, Position, TaskSpawner, Velocity},
	utils::{DebugAdapter, EventHandler, EventHandlerTypeProvider, HistoricalStorageExt},
};

use specs::{Entities, LazyUpdate, Read, ReadExpect, WriteStorage};

#[derive(Default)]
pub struct Cleanup;

#[derive(SystemDataCustom)]
pub struct CleanupData<'a> {
	entities: Entities<'a>,
	tasks: ReadExpect<'a, TaskSpawner>,
	lazy: Read<'a, LazyUpdate>,
	debug: DebugAdapter<'a>,

	is_zombie: WriteStorage<'a, IsZombie>,
}

impl EventHandlerTypeProvider for Cleanup {
	type Event = PowerupDespawnEvent;
}

impl<'a> EventHandler<'a> for Cleanup {
	type SystemData = CleanupData<'a>;

	fn on_event(&mut self, evt: &Self::Event, data: &mut CleanupData) {
		if !data.entities.is_alive(evt.mob) {
			return;
		}

		if data.is_zombie.mask().contains(evt.mob.id()) {
			data.debug.lazy_debug(evt.mob);
			return;
		}

		data.is_zombie
			.insert_with_history(evt.mob, IsZombie::from_sys(self))
			.unwrap();

		data.tasks.spawn(crate::task::delayed_delete(
			data.tasks.task_data(),
			evt.mob,
			ID_REUSE_TIME,
		));

		data.lazy.remove::<IsPowerup>(evt.mob);
		data.lazy.remove::<Mob>(evt.mob);
		data.lazy.remove::<Position>(evt.mob);
		data.lazy.remove::<Velocity>(evt.mob);
	}
}

system_info! {
	impl SystemInfo for Cleanup {
		type Dependencies = ();
	}
}

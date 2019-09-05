use specs::prelude::*;

use crate::component::channel::*;
use crate::component::event::*;
use crate::component::flag::*;
use crate::component::reference::PlayerRef;
use crate::types::collision::*;
use crate::types::*;
use crate::utils::{EventHandler, EventHandlerTypeProvider, HistoricalStorageExt};

use crate::consts::missile::ID_REUSE_TIME;

#[derive(Default)]
pub struct MissileHitSystem;

#[derive(SystemDataCustom)]
pub struct MissileHitSystemData<'a> {
	hit_channel: Write<'a, OnPlayerHit>,
	tasks: WriteExpect<'a, TaskSpawner>,
	lazy: Read<'a, LazyUpdate>,

	mob: ReadStorage<'a, Mob>,
	player_flag: ReadStorage<'a, IsPlayer>,
	entities: Entities<'a>,
	is_zombie: WriteStorage<'a, IsZombie>,

	despawn: Write<'a, OnMissileDespawn>,
}

impl EventHandlerTypeProvider for MissileHitSystem {
	type Event = PlayerMissileCollision;
}

impl<'a> EventHandler<'a> for MissileHitSystem {
	type SystemData = MissileHitSystemData<'a>;

	fn on_event(&mut self, evt: &Self::Event, data: &mut Self::SystemData) {
		let Collision(c1, c2) = evt.0;
		let player;
		let missile;

		match data.player_flag.get(c1.ent) {
			Some(_) => {
				player = c1;
				missile = c2;
			}
			None => {
				missile = c1;
				player = c2;
			}
		}

		if !data.entities.is_alive(missile.ent) {
			return;
		}
		if data.is_zombie.mask().contains(missile.ent.id()) {
			return;
		}

		// Remove a bunch of components that are used to
		// recognize missiles lazily (they will get
		// removed at the end of the frame)
		data.lazy.remove::<Position>(missile.ent);
		data.lazy.remove::<Mob>(missile.ent);
		data.lazy.remove::<IsMissile>(missile.ent);
		data.lazy.remove::<Velocity>(missile.ent);
		data.lazy.remove::<Team>(missile.ent);
		data.lazy.remove::<PlayerRef>(missile.ent);

		data.is_zombie
			.insert_with_history(missile.ent, IsZombie::from_sys(self))
			.unwrap();

		data.tasks.spawn(crate::task::delayed_delete(
			data.tasks.task_data(),
			missile.ent,
			ID_REUSE_TIME,
		));

		data.hit_channel.single_write(PlayerHit {
			player: player.ent,
			missile: missile.ent,
		});

		data.despawn.single_write(MissileDespawn {
			missile: missile.ent,
			pos: missile.pos,
			mob: *try_get!(missile.ent, data.mob),
			ty: MissileDespawnType::HitPlayer,
		});
	}
}

system_info! {
	impl SystemInfo for MissileHitSystem {
		type Dependencies = super::MissileFireHandler;
	}
}

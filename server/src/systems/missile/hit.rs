use crate::types::collision::*;
use crate::types::FutureDispatcher;
use crate::types::*;
use specs::prelude::*;

use crate::component::channel::*;
use crate::component::event::*;
use crate::component::flag::*;
use crate::component::reference::PlayerRef;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

use crate::consts::missile::ID_REUSE_TIME;
use crate::consts::timer::DELETE_ENTITY;

#[derive(Default)]
pub struct MissileHitSystem;

#[derive(SystemData)]
pub struct MissileHitSystemData<'a> {
	hit_channel: Write<'a, OnPlayerHit>,
	dispatch: ReadExpect<'a, FutureDispatcher>,
	lazy: Read<'a, LazyUpdate>,

	mob: ReadStorage<'a, Mob>,
	player_flag: ReadStorage<'a, IsPlayer>,
	entities: Entities<'a>,
	hitmarker: WriteStorage<'a, HitMarker>,

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
		if data.hitmarker.get(missile.ent).is_some() {
			return;
		}

		data.hitmarker.insert(missile.ent, HitMarker {}).unwrap();

		// Remove a bunch of components that are used to
		// recognize missiles lazily (they will get
		// removed at the end of the frame)
		data.lazy.remove::<Position>(missile.ent);
		data.lazy.remove::<Mob>(missile.ent);
		data.lazy.remove::<IsMissile>(missile.ent);
		data.lazy.remove::<Velocity>(missile.ent);
		data.lazy.remove::<Team>(missile.ent);
		data.lazy.remove::<PlayerRef>(missile.ent);

		data.dispatch
			.run_delayed(ID_REUSE_TIME, move |inst| TimerEvent {
				ty: *DELETE_ENTITY,
				instant: inst,
				data: Some(Box::new(missile.ent)),
			});

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

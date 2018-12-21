use specs::*;

use types::collision::Collision;
use types::*;

use component::channel::OnMissileDespawn;
use component::event::*;
use component::flag::IsMissile;
use component::reference::PlayerRef;
use utils::{EventHandler, EventHandlerTypeProvider};

use consts::missile::ID_REUSE_TIME;
use consts::timer::DELETE_ENTITY;

#[derive(Default)]
pub struct MissileExplodeSystem;

#[derive(SystemData)]
pub struct MissileExplodeSystemData<'a> {
	dispatch: ReadExpect<'a, FutureDispatcher>,
	lazy: Read<'a, LazyUpdate>,

	mob: ReadStorage<'a, Mob>,
	pos: ReadStorage<'a, Position>,
	channel: Write<'a, OnMissileDespawn>,
}

impl EventHandlerTypeProvider for MissileExplodeSystem {
	type Event = MissileTerrainCollision;
}

impl<'a> EventHandler<'a> for MissileExplodeSystem {
	type SystemData = MissileExplodeSystemData<'a>;

	fn on_event(&mut self, evt: &MissileTerrainCollision, data: &mut Self::SystemData) {
		let Collision(c1, c2) = evt.0;

		let missile_ent;

		if c1.ent.id() == 0 {
			missile_ent = c2.ent;
		} else {
			missile_ent = c1.ent;
		}

		// Remove a bunch of components that are used to
		// recognize missiles lazily (they will get
		// removed at the end of the frame)
		data.lazy.remove::<Position>(missile_ent);
		data.lazy.remove::<Mob>(missile_ent);
		data.lazy.remove::<IsMissile>(missile_ent);
		data.lazy.remove::<Velocity>(missile_ent);
		data.lazy.remove::<Team>(missile_ent);
		data.lazy.remove::<PlayerRef>(missile_ent);

		data.dispatch
			.run_delayed(*ID_REUSE_TIME, move |inst| TimerEvent {
				ty: *DELETE_ENTITY,
				instant: inst,
				data: Some(Box::new(missile_ent)),
			});

		data.channel.single_write(MissileDespawn {
			missile: missile_ent,
			pos: *try_get!(missile_ent, data.pos),
			mob: *try_get!(missile_ent, data.mob),
			ty: MissileDespawnType::HitTerrain,
		});
	}
}

use super::MissileTerrainCollisionSystem;
use dispatch::SystemInfo;

impl SystemInfo for MissileExplodeSystem {
	type Dependencies = MissileTerrainCollisionSystem;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

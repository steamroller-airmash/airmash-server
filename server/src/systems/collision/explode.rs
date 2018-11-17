use specs::*;

use types::collision::Collision;
use types::*;

use component::channel::*;
use component::reference::PlayerRef;
use component::flag::IsMissile;
use component::event::TimerEvent;

use consts::timer::DELETE_ENTITY;
use protocol::server::MobDespawnCoords;

use std::time::Duration;

pub struct MissileExplodeSystem {
	reader: Option<OnMissileTerrainCollisionReader>,
}

#[derive(SystemData)]
pub struct MissileExplodeSystemData<'a> {
	pub conns: Read<'a, Connections>,
	pub channel: Read<'a, OnMissileTerrainCollision>,
	pub entities: Entities<'a>,
	pub dispatch: ReadExpect<'a, FutureDispatcher>,
	pub lazy: Read<'a, LazyUpdate>,

	pub types: ReadStorage<'a, Mob>,
	pub pos: ReadStorage<'a, Position>,
}

impl MissileExplodeSystem {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for MissileExplodeSystem {
	type SystemData = MissileExplodeSystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(
			res.fetch_mut::<OnMissileTerrainCollision>()
				.register_reader(),
		);
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
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

			data.dispatch.run_delayed(Duration::from_secs(60), move |inst| {
				TimerEvent {
					ty: *DELETE_ENTITY,
					instant: inst,
					data: Some(Box::new(missile_ent))
				}
			});

			let packet = MobDespawnCoords {
				id: missile_ent.into(),
				ty: *data.types.get(missile_ent).unwrap(),
				pos: *data.pos.get(missile_ent).unwrap(),
			};

			data.conns.send_to_all(packet);
		}
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
		Self::new()
	}
}

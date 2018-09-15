use specs::*;
use types::collision::*;
use types::*;

use component::channel::*;
use component::event::PlayerHit;
use component::flag::*;
use component::reference::PlayerRef;

pub struct MissileHitSystem {
	reader: Option<OnPlayerMissileCollisionReader>,
}

#[derive(SystemData)]
pub struct MissileHitSystemData<'a> {
	pub channel: Read<'a, OnPlayerMissileCollision>,
	pub kill_channel: Write<'a, OnPlayerKilled>,
	pub hit_channel: Write<'a, OnPlayerHit>,
	pub config: Read<'a, Config>,
	pub conns: Read<'a, Connections>,

	pub health: WriteStorage<'a, Health>,
	pub plane: ReadStorage<'a, Plane>,
	pub upgrades: ReadStorage<'a, Upgrades>,
	pub owner: ReadStorage<'a, PlayerRef>,
	pub player_flag: ReadStorage<'a, IsPlayer>,
	pub entities: Entities<'a>,
	pub hitmarker: WriteStorage<'a, HitMarker>,

	pub mob: ReadStorage<'a, Mob>,
	pub pos: ReadStorage<'a, Position>,
}

impl MissileHitSystem {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for MissileHitSystem {
	type SystemData = MissileHitSystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(
			res.fetch_mut::<OnPlayerMissileCollision>()
				.register_reader(),
		);
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
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
				continue;
			}
			if data.hitmarker.get(missile.ent).is_some() {
				continue;
			}

			data.hitmarker.insert(missile.ent, HitMarker {}).unwrap();
			data.entities.delete(missile.ent).unwrap();

			data.hit_channel.single_write(PlayerHit {
				player: player.ent,
				missile: missile.ent,
			});
		}
	}
}

use super::*;
use dispatch::SystemInfo;

impl SystemInfo for MissileHitSystem {
	type Dependencies = MissileFireHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

use specs::*;
use types::collision::*;
use types::*;

use component::channel::*;
use component::event::PlayerKilled;
use component::reference::PlayerRef;

use protocol::server::{PlayerHit, PlayerHitPlayer};
use protocol::{to_bytes, ServerPacket};
use websocket::OwnedMessage;

pub struct MissileHitSystem {
	reader: Option<OnPlayerMissileCollisionReader>,
}

#[derive(SystemData)]
pub struct MissileHitSystemData<'a> {
	pub channel: Read<'a, OnPlayerMissileCollision>,
	pub kill_channel: Write<'a, OnPlayerKilled>,
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

			let plane = data.plane.get(player.ent).unwrap();
			let health = data.health.get_mut(player.ent).unwrap();
			let upgrades = data.upgrades.get(player.ent).unwrap();

			let mob = data.mob.get(missile.ent).unwrap();
			let pos = data.pos.get(missile.ent).unwrap();
			let owner = data.owner.get(missile.ent).unwrap();

			let ref planeconf = data.config.planes[*plane];
			let ref mobconf = data.config.mobs[*mob].missile.unwrap();
			let ref upgconf = data.config.upgrades;

			*health -= mobconf.damage * planeconf.damage_factor
				/ upgconf.defense.factor[upgrades.defense as usize];

			data.hitmarker.insert(missile.ent, HitMarker {}).unwrap();
			data.entities.delete(missile.ent).unwrap();

			info!(
				"{} {}",
				*health,
				mobconf.damage * planeconf.damage_factor
					/ upgconf.defense.factor[upgrades.defense as usize]
			);

			if health.inner() <= 0.0 {
				data.kill_channel.single_write(PlayerKilled {
					missile: missile.ent,
					player: player.ent,
					killer: owner.0,
					pos: *pos,
				});
			} 

			let packet = PlayerHit {
				id: missile.ent,
				owner: owner.0,
				pos: *pos,
				ty: *mob,
				players: vec![PlayerHitPlayer {
					id: player.ent,
					health: *health,
					health_regen: planeconf.health_regen,
				}],
			};

			data.conns.send_to_all(OwnedMessage::Binary(
				to_bytes(&ServerPacket::PlayerHit(packet)).unwrap(),
			));
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

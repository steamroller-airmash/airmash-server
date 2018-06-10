
use specs::*;
use types::*;

use component::time::{MobSpawnTime, ThisFrame};

use websocket::OwnedMessage;
use airmash_protocol::{to_bytes, ServerPacket};
use airmash_protocol::server::{MobDespawn};

pub struct MissileCull;

#[derive(SystemData)]
pub struct MissileCullData<'a> {
	pub ents: Entities<'a>,
	pub spawntime: ReadStorage<'a, MobSpawnTime>,
	pub mob: ReadStorage<'a, Mob>,
	pub config: Read<'a, Config>,
	pub thisframe: Read<'a, ThisFrame>,
	pub conns: Read<'a, Connections>
}

impl<'a> System<'a> for MissileCull {
	type SystemData = MissileCullData<'a>;

	fn run(&mut self, data: MissileCullData<'a>) {
		(&*data.ents, &data.mob, &data.spawntime).join()
			.filter_map(|(ent, mob, spawntime)| {
				let ref info = data.config.mobs[*mob];

				let dt = data.thisframe.0 - spawntime.0;

				if dt > info.lifetime {
					Some((ent, *mob))
				}
				else {
					None
				}
			})
			.for_each(|(ent, mob)| {
				data.ents.delete(ent).unwrap();

				let packet = MobDespawn {
					id: ent,
					ty: mob
				};

				data.conns.send_to_all(OwnedMessage::Binary(
					to_bytes(&ServerPacket::MobDespawn(packet)).unwrap()
				));
			});
	}
}

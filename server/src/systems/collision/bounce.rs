use shrev::*;
use specs::*;
use types::*;

use component::time::{ThisFrame, StartTime};

use systems::collision::bucket::Collision;

use airmash_protocol::server::EventBounce;
use airmash_protocol::{to_bytes, ServerPacket};
use websocket::OwnedMessage;

pub struct BounceSystem {
	reader: Option<ReaderId<Collision>>,
}

#[derive(SystemData)]
pub struct BounceSystemData<'a> {
	pub entity: Entities<'a>,
	pub vel: WriteStorage<'a, Velocity>,
	pub pos: ReadStorage<'a, Position>,
	pub rot: ReadStorage<'a, Rotation>,
	pub plane: ReadStorage<'a, Plane>,
	pub keystate: ReadStorage<'a, KeyState>,
	pub conns: Read<'a, Connections>,
	pub config: Read<'a, Config>,
	pub channel: Read<'a, EventChannel<Collision>>,
	pub thisframe: Read<'a, ThisFrame>,
	pub starttime: Read<'a, StartTime>,
}

impl BounceSystem {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for BounceSystem {
	type SystemData = BounceSystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		self.reader = Some(res.fetch_mut::<EventChannel<Collision>>().register_reader());

		Self::SystemData::setup(res);
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if evt.0.layer == 0 || evt.1.layer == 0  {
				assert!(evt.1.layer != evt.0.layer);

				let rel;
				let maxspd;
				let ent;
				if evt.0.layer == 0 {
					ent = evt.1.ent;
					rel = (evt.1.pos - evt.0.pos).normalized();
					maxspd = *data.vel.get(evt.1.ent).unwrap();
				} else {
					ent = evt.0.ent;
					rel = (evt.0.pos - evt.1.pos).normalized();
					maxspd = *data.vel.get(evt.0.ent).unwrap();
				};

				let vel = rel * Speed::max(maxspd.length(), Speed::new(1.0));

				match data.vel.get_mut(ent) {
					Some(v) => *v = vel,
					None => {
						warn!(
							target: "server",
							"EventBounce triggered for non-player entity {:?}",
							ent
						);
						continue;
					}
				}

				let pos = data.pos.get(ent).unwrap();
				let rot = data.rot.get(ent).unwrap();
				let keystate = data.keystate.get(ent).unwrap();
				let plane = data.plane.get(ent).unwrap();
				let state = keystate.to_server(&plane);

				let packet = EventBounce {
					clock: (data.thisframe.0 - data.starttime.0).to_clock(),
					id: ent,
					pos: *pos,
					rot: *rot,
					speed: vel,
					keystate: state,
				};

				data.conns.send_to_all(OwnedMessage::Binary(
					to_bytes(&ServerPacket::EventBounce(packet)).unwrap(),
				));
			}
		}
	}
}

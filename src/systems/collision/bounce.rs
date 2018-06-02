
use specs::*;
use shrev::*;
use types::*;

use systems::collision::bucket::Collision;

use airmash_protocol::{to_bytes, ServerPacket};
use airmash_protocol::server::EventBounce;

pub struct BounceSystem {
	reader: ReaderId<EventChannel<Collision>>
}

#[derive(SystemData)]
pub struct BounceSystemData<'a> {
	pub entity:  Entities<'a>,
	pub vel:     WriteStorage<'a, Velocity>,
	pub channel: Read<'a, EventChannel<Collision>>
}

impl<'a> System<'a> for BounceSystem {
	type SystemData = BounceSystemData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {

		
	}
}
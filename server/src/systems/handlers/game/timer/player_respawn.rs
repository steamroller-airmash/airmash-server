
use specs::*;

use std::any::Any;

use types::*;

use consts::timer::*;
use component::flag::*;
use component::channel::*;

use SystemInfo;
use systems::TimerHandler;

use OwnedMessage;
use protocol::{to_bytes, ServerPacket};
use protocol::Upgrades as ProtocolUpgrades;
use protocol::server::PlayerRespawn;

pub struct PlayerRespawnSystem {
	reader: Option<OnTimerEventReader>
}

#[derive(SystemData)]
pub struct PlayerRespawnSystemData<'a> {
	pub channel: Read<'a, OnTimerEvent>,
	pub conns: Read<'a, Connections>,

	pub pos: WriteStorage<'a, Position>,
	pub vel: WriteStorage<'a, Velocity>,
	pub rot: WriteStorage<'a, Rotation>,
	pub health: WriteStorage<'a, Health>,
	pub energy: WriteStorage<'a, Energy>,

	pub is_dead: WriteStorage<'a, IsDead>,
	pub is_spec: ReadStorage<'a, IsSpectating>,

}

impl<'a> System<'a> for PlayerRespawnSystem {
	type SystemData = PlayerRespawnSystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(
			res.fetch_mut::<OnTimerEvent>().register_reader()
		);
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if evt.ty != *RESPAWN_TIME { continue; }

			let player = match evt.data {
				Some(ref dat) => match (*dat).downcast_ref::<Entity>() {
					Some(val) => *val,
					None => {
						error!("Unable to downcast TimerEvent data to Entity! Event will be skipped.");
						continue;
					}	
				},
				None => continue,
			};

			*data.pos.get_mut(player).unwrap() = Position::default();
			*data.vel.get_mut(player).unwrap() = Velocity::default();
			*data.rot.get_mut(player).unwrap() = Rotation::default();
			*data.health.get_mut(player).unwrap() = Health::new(1.0);
			*data.energy.get_mut(player).unwrap() = Energy::new(1.0);
			data.is_dead.remove(player);

			if data.is_spec.get(player).is_none() {
				data.conns.send_to_all(OwnedMessage::Binary(
					to_bytes(&ServerPacket::PlayerRespawn(PlayerRespawn {
						id: player,
						pos: *data.pos.get(player).unwrap(),
						rot: *data.rot.get(player).unwrap(),
						upgrades: ProtocolUpgrades::default(),
					})).unwrap(),
				));
			}
		}
	}
}

impl SystemInfo for PlayerRespawnSystem {
	type Dependencies = TimerHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new(_: Box<Any>) -> Self {
		Self { reader: None }
	}
}

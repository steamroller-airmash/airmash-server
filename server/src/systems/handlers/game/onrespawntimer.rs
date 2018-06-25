use specs::*;
use types::*;

use consts::timer::RESPAWN_TIME;
use OwnedMessage;
use SystemInfo;

use component::channel::{OnTimerEvent, OnTimerEventReader};
use component::flag::{IsDead, IsSpectating};

use protocol::server::PlayerRespawn;
use protocol::{to_bytes, ServerPacket, Upgrades as ProtoUpgrades};

use systems::TimerHandler;

use std::any::Any;

pub struct OnRespawnTimer {
	reader: Option<OnTimerEventReader>,
}

impl OnRespawnTimer {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

#[derive(SystemData)]
pub struct OnRespawnTimerData<'a> {
	pub gamemode: GameModeWriter<'a, GameMode>,
	pub channel: Read<'a, OnTimerEvent>,
	pub conns: Read<'a, Connections>,

	pub entities: Entities<'a>,
	pub isspec: ReadStorage<'a, IsSpectating>,
	pub isdead: WriteStorage<'a, IsDead>,
	pub team: ReadStorage<'a, Team>,

	pub pos: WriteStorage<'a, Position>,
	pub rot: WriteStorage<'a, Rotation>,
	pub vel: WriteStorage<'a, Velocity>,
	pub upgrades: ReadStorage<'a, Upgrades>,
	pub powerups: ReadStorage<'a, Powerups>,
}

impl<'a> System<'a> for OnRespawnTimer {
	type SystemData = OnRespawnTimerData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnTimerEvent>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if evt.ty != *RESPAWN_TIME {
				continue;
			}

			let player: Entity = *evt.data.as_ref().unwrap().downcast_ref().unwrap();

			if !data.entities.is_alive(player) {
				continue;
			}

			data.isdead.remove(player).unwrap();

			if data.isspec.get(player).is_some() {
				continue;
			}

			let pos = data
				.gamemode
				.get_mut()
				.respawn_pos(player, *data.team.get(player).unwrap());

			*data.pos.get_mut(player).unwrap() = pos;
			*data.vel.get_mut(player).unwrap() = Velocity::default();
			*data.rot.get_mut(player).unwrap() = Rotation::default();

			let upgrades = data.upgrades.get(player).unwrap();
			let powerups = data.powerups.get(player).unwrap();

			// TODO: Players should get a shield when respawning
			let packet = PlayerRespawn {
				id: player,
				pos: pos,
				rot: Rotation::default(),
				upgrades: ProtoUpgrades {
					speed: upgrades.speed,
					shield: powerups.shield,
					inferno: powerups.inferno,
				},
			};

			data.conns.send_to_all(OwnedMessage::Binary(
				to_bytes(&ServerPacket::PlayerRespawn(packet)).unwrap(),
			));
		}
	}
}

impl SystemInfo for OnRespawnTimer {
	type Dependencies = TimerHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new(_: Box<Any>) -> Self {
		Self::new()
	}
}

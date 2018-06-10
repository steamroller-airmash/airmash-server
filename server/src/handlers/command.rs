use shrev::*;
use specs::*;
use types::*;

use airmash_protocol::server::{
	PlayerType,
	PlayerRespawn,
	PlayerFlag,
	ServerMessage
};
use airmash_protocol::client::Command;
use airmash_protocol::{to_bytes, ServerPacket, FlagCode, Upgrades as ProtocolUpgrades};
use websocket::OwnedMessage;

pub struct CommandHandler {
	reader: Option<ReaderId<(ConnectionId, Command)>>,
}

#[derive(SystemData)]
pub struct CommandHandlerData<'a> {
	channel: Read<'a, EventChannel<(ConnectionId, Command)>>,
	conns: Read<'a, Connections>,
	planes: WriteStorage<'a, Plane>,
	flags: WriteStorage<'a, Flag>,

	pos: WriteStorage<'a, Position>,
	rot: WriteStorage<'a, Rotation>,
	vel: WriteStorage<'a, Velocity>
}

impl CommandHandler {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for CommandHandler {
	type SystemData = CommandHandlerData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(
			res.fetch_mut::<EventChannel<(ConnectionId, Command)>>()
				.register_reader(),
		);
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let player = match data.conns.0[&evt.0].player {
				Some(p) => p,
				None => continue
			};

			let packet;

			if evt.1.com == "flag" {
				let flag = Flag::from_str(&evt.1.data).unwrap_or(FlagCode::UnitedNations);

				packet = ServerPacket::PlayerFlag(PlayerFlag {
					id: player,
					flag: flag
				});

				*data.flags.get_mut(player).unwrap() = flag;
			}
			else if evt.1.com == "respawn" {
				let num = match evt.1.data.parse() {
					Ok(n) => n,
					Err(_) => continue
				};
				let ty = match Plane::try_from(num) {
					Some(n) => n,
					None => continue
				};

				*data.pos.get_mut(player).unwrap() = Position::default();
				*data.vel.get_mut(player).unwrap() = Velocity::default();
				*data.rot.get_mut(player).unwrap() = Rotation::default();
				*data.planes.get_mut(player).unwrap() = ty;

				data.conns.send_to_all(OwnedMessage::Binary(
					to_bytes(&ServerPacket::PlayerRespawn(PlayerRespawn {
						id: player,
						pos: *data.pos.get(player).unwrap(),
						rot: *data.rot.get(player).unwrap(),
						upgrades: ProtocolUpgrades::default()
					})).unwrap()
				));

				packet = ServerPacket::PlayerType(PlayerType {
					id: player,
					ty: ty
				});
			}
			else {
				continue;
			}

			data.conns.send_to_all(OwnedMessage::Binary(
				to_bytes(&packet).unwrap()
			));
		}
	}
}

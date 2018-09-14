use shrev::*;
use specs::*;
use types::*;

use component::channel::*;
use component::event::*;
use component::flag::*;

use protocol::client::Command;
use protocol::server::{PlayerFlag, PlayerType};
use protocol::{FlagCode, ServerPacket};

use std::convert::TryFrom;
use std::str::FromStr;

pub struct CommandHandler {
	reader: Option<OnCommandReader>,
}

#[derive(SystemData)]
pub struct CommandHandlerData<'a> {
	channel: Read<'a, OnCommand>,
	respawn_channel: Write<'a, OnPlayerRespawn>,
	conns: Read<'a, Connections>,
	planes: WriteStorage<'a, Plane>,
	flags: WriteStorage<'a, FlagCode>,
	isspec: WriteStorage<'a, IsSpectating>,
	health: ReadStorage<'a, Health>,
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
				None => continue,
			};

			let packet;

			if evt.1.com == "flag" {
				let flag = FlagCode::from_str(&evt.1.data).unwrap_or(FlagCode::UnitedNations);

				packet = ServerPacket::PlayerFlag(PlayerFlag {
					id: player.into(),
					flag: flag,
				});

				*data.flags.get_mut(player).unwrap() = flag;
			} else if evt.1.com == "respawn" {
				let num: i32 = match evt.1.data.parse() {
					Ok(n) => n,
					Err(_) => continue,
				};
				let ty = match Plane::try_from(num) {
					Ok(n) => n,
					_ => continue,
				};

				// Make sure player health is full before allowing respawn
				match data.health.get(player) {
					Some(&health) => {
						if health < Health::new(1.0) {
							// TODO: Actual number is slightly lower than 1.0?
							continue;
						}
					}
					_ => continue,
				}

				*data.planes.get_mut(player).unwrap() = ty;
				data.isspec.remove(player);

				data.respawn_channel.single_write(PlayerRespawn { player });

				packet = ServerPacket::PlayerType(PlayerType {
					id: player.into(),
					ty: ty,
				});
			} else {
				continue;
			}

			data.conns.send_to_all(packet);
		}
	}
}

use handlers::OnCloseHandler;
use SystemInfo;

impl SystemInfo for CommandHandler {
	type Dependencies = OnCloseHandler;

	fn new() -> Self {
		Self::new()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

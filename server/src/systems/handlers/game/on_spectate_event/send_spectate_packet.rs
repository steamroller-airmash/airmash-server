use specs::*;
use types::*;

use component::channel::*;
use protocol::server::GameSpectate;

use SystemInfo;

pub struct SendSpectatePacket {
	reader: Option<OnPlayerSpectateReader>,
}

#[derive(SystemData)]
pub struct SendSpectatePacketData<'a> {
	pub channel: Read<'a, OnPlayerSpectate>,
	pub conns: Read<'a, Connections>,
}

impl<'a> System<'a> for SendSpectatePacket {
	type SystemData = SendSpectatePacketData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerSpectate>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			// GameSpectate only gets sent if there
			// is someone to spectate
			if evt.target.is_none() {
				continue;
			}

			let packet = GameSpectate {
				id: evt.target.unwrap().into(),
			};

			data.conns.send_to_player(evt.player, packet);
		}
	}
}

impl SystemInfo for SendSpectatePacket {
	type Dependencies = super::KnownEventSources;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

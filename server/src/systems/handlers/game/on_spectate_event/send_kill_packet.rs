use specs::*;
use types::*;

use component::channel::*;
use protocol::server::PlayerKill;

use SystemInfo;

pub struct SendKillPacket {
	reader: Option<OnPlayerSpectateReader>,
}

#[derive(SystemData)]
pub struct SendKillPacketData<'a> {
	pub channel: Read<'a, OnPlayerSpectate>,
	pub conns: Read<'a, Connections>,
}

impl<'a> System<'a> for SendKillPacket {
	type SystemData = SendKillPacketData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerSpectate>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			// If they are already (in spec/dead)
			// we don't need to despawn their plane
			if evt.is_dead || evt.is_spec {
				continue;
			}

			// Setting pos to Position::default()
			// indicates to the client that this
			// was a player going into spec.
			let packet = PlayerKill {
				id: evt.player.into(),
				killer: None,
				pos: Position::default(),
			};

			data.conns.send_to_player(evt.player, packet);
		}
	}
}

impl SystemInfo for SendKillPacket {
	type Dependencies = super::KnownEventSources;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

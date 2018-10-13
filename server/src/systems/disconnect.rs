use specs::*;

use fnv::FnvHashMap;

use std::time::{Duration, Instant};

use component::channel::*;
use component::time::ThisFrame;
use types::event::ConnectionClose;
use types::ConnectionId;
use utils::maybe_init::MaybeInit;
use SystemInfo;

use systems::PacketHandler;

const NO_PACKET_TIMEOUT: Duration = Duration::from_secs(10);

/// Get rid of players that are no longer connected.
///
/// Specifically, this is targetted at players that
/// disconnected without sending a close packet.
/// (Lost internet connection, malicious websocket
/// implementations, etc.). But it will also disconnect
/// clients that just don't send any packets either.
///
/// The timeout that is enforced for players is 10 seconds
/// without a packet. Since the default client blasts the
/// server with an ack packet every 50ms this will only
/// affect disconnected clients and improperly written
/// bot clients.
#[derive(Default)]
pub struct Disconnect {
	message: MaybeInit<OnMessageReader>,
	close: MaybeInit<OnCloseReader>,

	counts: FnvHashMap<ConnectionId, Instant>,
}

#[derive(SystemData)]
pub struct DisconnectData<'a> {
	this_frame: Read<'a, ThisFrame>,

	on_message: Read<'a, OnMessage>,
	on_close: Write<'a, OnClose>,
}

impl<'a> System<'a> for Disconnect {
	type SystemData = DisconnectData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.message = MaybeInit::init(res.fetch_mut::<OnMessage>().register_reader());
		self.close = MaybeInit::init(res.fetch_mut::<OnClose>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		let this_frame = data.this_frame.0;

		for evt in data.on_message.read(&mut self.message) {
			self.counts.insert(evt.conn, this_frame);
		}

		for evt in data.on_close.read(&mut self.close) {
			self.counts.remove(&evt.conn);
		}

		let iter = self
			.counts
			.iter()
			.filter(|(_, inst)| this_frame - **inst > NO_PACKET_TIMEOUT)
			.map(|(conn, _)| *conn);

		for conn in iter {
			data.on_close.single_write(ConnectionClose { conn });
		}
	}
}

impl SystemInfo for Disconnect {
	type Dependencies = PacketHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

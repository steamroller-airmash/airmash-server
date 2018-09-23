use shrev::*;
use specs::*;
use types::*;

use protocol::client::Key;
use protocol::KeyCode;

use component::time::{LastKeyTime, ThisFrame};

pub struct KeyHandler {
	reader: Option<ReaderId<(ConnectionId, Key)>>,
}

impl KeyHandler {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

#[derive(SystemData)]
pub struct KeyHandlerData<'a> {
	channel: Read<'a, EventChannel<(ConnectionId, Key)>>,
	conns: Read<'a, Connections>,
	this_frame: Read<'a, ThisFrame>,

	keystate: WriteStorage<'a, KeyState>,
	last_key: WriteStorage<'a, LastKeyTime>,
}

impl<'a> System<'a> for KeyHandler {
	type SystemData = KeyHandlerData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		self.reader = Some(
			res.fetch_mut::<EventChannel<(ConnectionId, Key)>>()
				.register_reader(),
		);

		Self::SystemData::setup(res);
	}

	fn run(&mut self, mut data: Self::SystemData) {
		if let Some(ref mut reader) = self.reader {
			for evt in data.channel.read(reader) {
				let player = match data.conns.0.get(&evt.0) {
					Some(data) => match data.player {
						Some(player) => player,
						None => continue,
					},
					None => continue,
				};

				let keystate = data.keystate.get_mut(player).unwrap();

				debug!(
					target: "server",
					"Received key {:?}",
					evt
				);

				data.last_key
					.insert(player, LastKeyTime(data.this_frame.0))
					.unwrap();

				match evt.1.key {
					KeyCode::Up => keystate.up = evt.1.state,
					KeyCode::Down => keystate.down = evt.1.state,
					KeyCode::Left => keystate.left = evt.1.state,
					KeyCode::Right => keystate.right = evt.1.state,
					KeyCode::Fire => keystate.fire = evt.1.state,
					KeyCode::Special => keystate.special = evt.1.state,
				}
			}
		}
	}
}

use dispatch::SystemInfo;
use handlers::OnCloseHandler;

impl SystemInfo for KeyHandler {
	type Dependencies = OnCloseHandler;

	fn new() -> Self {
		Self::new()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

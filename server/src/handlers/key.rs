use crate::types::*;
use shrev::*;
use specs::*;

use crate::protocol::client::Key;
use crate::protocol::KeyCode;

use crate::component::flag::ForcePlayerUpdate;
use crate::component::time::{LastKeyTime, ThisFrame};

#[derive(Default)]
pub struct KeyHandler {
	reader: Option<ReaderId<(ConnectionId, Key)>>,
}

#[derive(SystemData)]
pub struct KeyHandlerData<'a> {
	channel: Read<'a, EventChannel<(ConnectionId, Key)>>,
	conns: Read<'a, Connections>,
	this_frame: Read<'a, ThisFrame>,

	force: WriteStorage<'a, ForcePlayerUpdate>,
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
				let player = match data.conns.associated_player(evt.0) {
					Some(player) => player,
					None => continue,
				};

				let keystate = try_get!(player, mut data.keystate);

				debug!(
					target: "server",
					"Received key {:?}",
					evt
				);

				data.last_key
					.insert(player, LastKeyTime(data.this_frame.0))
					.unwrap();
				data.force.insert(player, ForcePlayerUpdate).unwrap();

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

system_info! {
	impl SystemInfo for KeyHandler {
		type Dependencies = super::OnCloseHandler;
	}
}

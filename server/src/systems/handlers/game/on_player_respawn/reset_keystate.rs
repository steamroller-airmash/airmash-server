use specs::*;

use component::channel::*;
use types::*;
use SystemInfo;

use systems::handlers::command::AllCommandHandlers;
use systems::handlers::game::on_join::AllJoinHandlers;

/// Reset the keystate of a player when they
/// respawn.
#[derive(Default)]
pub struct ResetKeyState {
	reader: Option<OnPlayerRespawnReader>,
}

#[derive(SystemData)]
pub struct ResetKeyStateData<'a> {
	channel: Read<'a, OnPlayerRespawn>,
	keystate: WriteStorage<'a, KeyState>,
}

impl<'a> System<'a> for ResetKeyState {
	type SystemData = ResetKeyStateData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerRespawn>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			*data.keystate.get_mut(evt.player).unwrap() = KeyState::default();
		}
	}
}

impl SystemInfo for ResetKeyState {
	type Dependencies = (AllJoinHandlers, AllCommandHandlers);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

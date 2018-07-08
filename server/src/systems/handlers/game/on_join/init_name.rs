use specs::*;

use types::*;

use SystemInfo;

use systems::handlers::packet::LoginHandler;

use component::channel::*;

pub struct InitName {
	reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct InitNameData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,

	pub names: WriteStorage<'a, Name>,
}

impl<'a> System<'a> for InitName {
	type SystemData = InitNameData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			data.names.insert(evt.id, evt.name.clone()).unwrap();
		}
	}
}

impl SystemInfo for InitName {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

use specs::*;

use types::*;

use SystemInfo;

use systems::handlers::packet::LoginHandler;

use component::channel::*;

pub struct InitConnection {
	reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct InitConnectionData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,

	pub conns: Write<'a, Connections>,
	pub associated: WriteStorage<'a, AssociatedConnection>,
}

impl<'a> System<'a> for InitConnection {
	type SystemData = InitConnectionData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			data.conns
				.associate(evt.conn, evt.id, ConnectionType::Primary);
			data.associated
				.insert(evt.id, AssociatedConnection(evt.conn))
				.unwrap();
		}
	}
}

impl SystemInfo for InitConnection {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

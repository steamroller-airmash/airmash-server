use specs::*;

use component::channel::*;
use consts::timer::*;
use types::*;

use protocol::client::Login;
use protocol::server::Error;
use protocol::ErrorType;

// Login needs write access to just
// about everything
#[derive(SystemData)]
pub struct LoginSystemData<'a> {
	pub conns: Read<'a, Connections>,
}

pub struct LoginFailed {
	reader: Option<OnTimerEventReader>,
}

impl LoginFailed {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for LoginFailed {
	type SystemData = (Read<'a, OnTimerEvent>, LoginSystemData<'a>);

	fn setup(&mut self, res: &mut Resources) {
		self.reader = Some(res.fetch_mut::<OnTimerEvent>().register_reader());

		Self::SystemData::setup(res);
	}

	fn run(&mut self, (channel, data): Self::SystemData) {
		for evt in channel.read(self.reader.as_mut().unwrap()) {
			if evt.ty != *LOGIN_FAILED {
				continue;
			}

			let evt = match evt.data {
				Some(ref v) => match (*v).downcast_ref::<(ConnectionId, Login)>() {
					Some(v) => v.clone(),
					None => continue,
				},
				None => continue,
			};

			data.conns.send_to(
				evt.0,
				Error {
					error: ErrorType::Banned,
				},
			);
			data.conns.close(evt.0);
		}
	}
}

use dispatch::SystemInfo;
use handlers::OnCloseHandler;

impl SystemInfo for LoginFailed {
	type Dependencies = OnCloseHandler;

	fn new() -> Self {
		Self::new()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

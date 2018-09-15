use specs::*;
use types::*;

use SystemInfo;

use consts::timer::DELAYED_MESSAGE;

use component::channel::{OnTimerEvent, OnTimerEventReader};

use protocol::server::ServerMessage;

#[derive(Default)]
pub struct DelayMessage {
	reader: Option<OnTimerEventReader>,
}

#[derive(SystemData)]
pub struct DelayMessageData<'a> {
	channel: Read<'a, OnTimerEvent>,
	conns: Read<'a, Connections>,
}

impl<'a> System<'a> for DelayMessage {
	type SystemData = DelayMessageData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnTimerEvent>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if evt.ty != *DELAYED_MESSAGE {
				continue;
			}

			let packet = match evt.data {
				Some(ref v) => match (*v).downcast_ref::<ServerMessage>() {
					Some(v) => v.clone(),
					None => {
						error!(
							target: "airmash:timer-datatype",
							"Timer event data type was not protocol::ServerMessage. It will be skipped!"
						);
						continue;
					}
				},
				None => continue,
			};

			data.conns.send_to_all(packet);
		}
	}
}

impl SystemInfo for DelayMessage {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

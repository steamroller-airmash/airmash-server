use component::event::TimerEvent;
use consts::timer::DELAYED_MESSAGE;
use protocol::server::ServerMessage;
use types::systemdata::*;
use utils::*;
use SystemInfo;

#[derive(Default)]
pub struct DelayMessage;

#[derive(SystemData)]
pub struct DelayMessageData<'a> {
	conns: SendToAll<'a>,
}

impl EventHandlerTypeProvider for DelayMessage {
	type Event = TimerEvent;
}

impl<'a> EventHandler<'a> for DelayMessage {
	type SystemData = DelayMessageData<'a>;

	fn on_event(&mut self, evt: &TimerEvent, data: &mut Self::SystemData) {
		if evt.ty != *DELAYED_MESSAGE {
			return;
		}

		let packet = match evt.data {
			Some(ref v) => match (*v).downcast_ref::<ServerMessage>() {
				Some(v) => v.clone(),
				None => {
					error!(
						target: "airmash:timer-datatype",
						"Timer event data type was not protocol::ServerMessage. It will be skipped!"
					);
					return;
				}
			},
			None => return,
		};

		data.conns.send_to_all(packet);
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

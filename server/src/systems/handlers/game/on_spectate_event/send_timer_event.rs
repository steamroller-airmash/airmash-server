use specs::*;

use types::*;

use component::channel::*;
use component::event::TimerEvent;
use component::time::*;
use consts::timer::SCORE_BOARD;

use SystemInfo;

pub struct SendTimerEvent {
	reader: Option<OnPlayerSpectateReader>,
}

#[derive(SystemData)]
pub struct SendTimerEventData<'a> {
	pub channel: Read<'a, OnPlayerSpectate>,
	pub conns: Read<'a, Connections>,
	pub timerchannel: Write<'a, OnTimerEvent>,

	pub thisframe: Read<'a, ThisFrame>,
}

impl<'a> System<'a> for SendTimerEvent {
	type SystemData = SendTimerEventData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerSpectate>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			// No need to inform clients that they are in
			// spec if they are already in spec
			if evt.is_dead || evt.is_spec {
				continue;
			}

			// The way that a plane disappearing
			// appears to be communicated back to
			// the client is by sending a scoreboard
			// update, this triggers that by writing
			// a scoreboard timer event. Scoreboard
			// should most likely get a dedicated
			// event channel in the future.
			let timer_evt = TimerEvent {
				ty: *SCORE_BOARD,
				instant: data.thisframe.0,
				data: None,
			};

			data.timerchannel.single_write(timer_evt);
		}
	}
}

impl SystemInfo for SendTimerEvent {
	type Dependencies = super::KnownEventSources;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

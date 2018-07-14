use specs::*;

use types::*;

use component::channel::*;
use component::event::*;
use component::flag::*;
use consts::timer::*;

use systems::TimerHandler;
use SystemInfo;

pub struct UnthrottlePlayer {
	reader: Option<OnTimerEventReader>,
}

#[derive(SystemData)]
pub struct UnthrottlePlayerData<'a> {
	pub channel: Read<'a, OnTimerEvent>,
	pub conns: Read<'a, Connections>,

	pub entities: Entities<'a>,
	pub throttled: WriteStorage<'a, IsChatThrottled>,
}

impl<'a> System<'a> for UnthrottlePlayer {
	type SystemData = UnthrottlePlayerData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnTimerEvent>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if evt.ty != *UNTHROTTLE_TIME {
				continue;
			}

			let evt: PlayerThrottle = match evt.data {
				Some(ref dat) => match (*dat).downcast_ref::<PlayerThrottle>() {
					Some(val) => val.clone(),
					None => {
						error!("Unable to downcast TimerEvent data to PlayerThrottle!");
						continue;
					}
				},
				None => continue,
			};

			if !data.entities.is_alive(evt.player) {
				continue;
			}

			data.throttled.remove(evt.player);
		}
	}
}

impl SystemInfo for UnthrottlePlayer {
	type Dependencies = TimerHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

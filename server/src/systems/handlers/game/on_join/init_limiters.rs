use specs::*;

use types::*;

use SystemInfo;

use systems::handlers::packet::LoginHandler;

use component::channel::*;
use component::ratelimit::*;

use std::time::Duration;

const THROTTLE_LIMIT: usize = 2;
const MUTE_LIMIT: usize = 15;

lazy_static! {
	static ref THROTTLE_PERIOD: Duration = Duration::from_secs(4);
	static ref MUTE_PERIOD: Duration = Duration::from_secs(60);
}

pub struct InitLimiters {
	reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct InitLimitersData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,

	pub mute: WriteStorage<'a, ChatMuteLimiter>,
	pub throttle: WriteStorage<'a, ChatThrottleLimiter>,
}

impl<'a> System<'a> for InitLimiters {
	type SystemData = InitLimitersData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			data.mute
				.insert(
					evt.id,
					ChatMuteLimiter(RateLimiter::new(MUTE_LIMIT, *MUTE_PERIOD)),
				).unwrap();

			data.throttle
				.insert(
					evt.id,
					ChatThrottleLimiter(RateLimiter::new(THROTTLE_LIMIT, *THROTTLE_PERIOD)),
				).unwrap();
		}
	}
}

impl SystemInfo for InitLimiters {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

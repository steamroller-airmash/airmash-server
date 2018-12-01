use specs::*;

use types::*;

use SystemInfo;

use systems::handlers::packet::LoginHandler;

use component::event::*;
use component::ratelimit::*;
use utils::{EventHandler, EventHandlerTypeProvider};

use std::time::Duration;

const THROTTLE_LIMIT: usize = 2;
const MUTE_LIMIT: usize = 15;

lazy_static! {
	static ref THROTTLE_PERIOD: Duration = Duration::from_secs(4);
	static ref MUTE_PERIOD: Duration = Duration::from_secs(60);
}

#[derive(Default)]
pub struct InitLimiters;

#[derive(SystemData)]
pub struct InitLimitersData<'a> {
	mute: WriteStorage<'a, ChatMuteLimiter>,
	throttle: WriteStorage<'a, ChatThrottleLimiter>,
}

impl EventHandlerTypeProvider for InitLimiters {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for InitLimiters {
	type SystemData = InitLimitersData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		data.mute
			.insert(
				evt.id,
				ChatMuteLimiter(RateLimiter::new(MUTE_LIMIT, *MUTE_PERIOD)),
			)
			.unwrap();

		data.throttle
			.insert(
				evt.id,
				ChatThrottleLimiter(RateLimiter::new(THROTTLE_LIMIT, *THROTTLE_PERIOD)),
			)
			.unwrap();
	}
}

impl SystemInfo for InitLimiters {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

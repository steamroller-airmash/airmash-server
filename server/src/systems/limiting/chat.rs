use specs::*;
use types::*;

use SystemInfo;

use component::channel::*;
use component::event::*;
use component::flag::*;
use component::ratelimit::*;
use component::time::ThisFrame;

use systems::handlers::game::on_join::InitLimiters;
use systems::handlers::packet::ChatEventHandler;

pub struct LimitChat {
	reader: Option<OnChatEventReader>,
}

#[derive(SystemData)]
pub struct LimitChatData<'a> {
	channel: Read<'a, OnAnyChatEvent>,
	conns: Read<'a, Connections>,

	throttle: WriteStorage<'a, ChatThrottleLimiter>,
	mute: WriteStorage<'a, ChatMuteLimiter>,

	throttle_channel: Write<'a, OnPlayerThrottled>,
	mute_channel: Write<'a, OnPlayerMuted>,

	this_frame: Read<'a, ThisFrame>,

	is_throttled: WriteStorage<'a, IsChatThrottled>,
	is_muted: WriteStorage<'a, IsChatMuted>,
}

impl<'a> System<'a> for LimitChat {
	type SystemData = LimitChatData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnAnyChatEvent>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		let now = data.this_frame.0;

		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let player = match data.conns.associated_player(evt.conn) {
				Some(p) => p,
				None => continue,
			};

			let throttle = data.throttle.get_mut(player).unwrap();
			throttle.0.add_event(now);

			let mute = data.mute.get_mut(player).unwrap();
			mute.0.add_event(now);

			if throttle.0.limit_reached() {
				data.is_throttled.insert(player, IsChatThrottled).unwrap();
				data.throttle_channel
					.single_write(PlayerThrottle { player });
			}

			if mute.0.limit_reached() {
				data.is_muted.insert(player, IsChatMuted).unwrap();
				data.mute_channel.single_write(PlayerMute { player });
			}
		}
	}
}

impl SystemInfo for LimitChat {
	type Dependencies = (ChatEventHandler, InitLimiters);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

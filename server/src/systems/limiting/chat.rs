use specs::prelude::*;

use crate::component::channel::*;
use crate::component::event::*;
use crate::component::flag::*;
use crate::component::ratelimit::*;
use crate::component::time::ThisFrame;
use crate::types::*;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

// use crate::systems::handlers::game::on_join::InitLimiters;
use crate::systems::handlers::packet::ChatEventHandler;

#[derive(Default)]
pub struct LimitChat;

#[derive(SystemData)]
pub struct LimitChatData<'a> {
	conns: Read<'a, Connections>,

	throttle: WriteStorage<'a, ChatThrottleLimiter>,
	mute: WriteStorage<'a, ChatMuteLimiter>,

	throttle_channel: Write<'a, OnPlayerThrottled>,
	mute_channel: Write<'a, OnPlayerMuted>,

	this_frame: Read<'a, ThisFrame>,

	is_throttled: WriteStorage<'a, IsChatThrottled>,
	is_muted: WriteStorage<'a, IsChatMuted>,
}

impl EventHandlerTypeProvider for LimitChat {
	type Event = AnyChatEvent;
}

impl<'a> EventHandler<'a> for LimitChat {
	type SystemData = LimitChatData<'a>;

	fn on_event(&mut self, evt: &AnyChatEvent, data: &mut Self::SystemData) {
		let now = data.this_frame.0;

		let player = match data.conns.associated_player(evt.conn) {
			Some(p) => p,
			None => return,
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

system_info! {
	impl SystemInfo for LimitChat {
		type Dependencies = (
			ChatEventHandler,
			// InitLimiters
		);
	}
}

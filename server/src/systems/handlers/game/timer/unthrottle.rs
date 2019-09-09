use specs::prelude::*;

use crate::component::event::*;
use crate::component::flag::*;
use crate::consts::timer::*;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct UnthrottlePlayer;

#[derive(SystemDataCustom)]
pub struct UnthrottlePlayerData<'a> {
	entities: Entities<'a>,
	throttled: WriteStorage<'a, IsChatThrottled>,
}

impl EventHandlerTypeProvider for UnthrottlePlayer {
	type Event = TimerEvent;
}

impl<'a> EventHandler<'a> for UnthrottlePlayer {
	type SystemData = UnthrottlePlayerData<'a>;

	fn on_event(&mut self, evt: &TimerEvent, data: &mut Self::SystemData) {
		if evt.ty != *UNTHROTTLE_TIME {
			return;
		}

		let evt: PlayerThrottle = match evt.data {
			Some(ref dat) => match (*dat).downcast_ref::<PlayerThrottle>() {
				Some(val) => val.clone(),
				None => {
					error!("Unable to downcast TimerEvent data to PlayerThrottle!");
					return;
				}
			},
			None => return,
		};

		if !data.entities.is_alive(evt.player) {
			return;
		}

		data.throttled.remove(evt.player);
	}
}

system_info! {
	impl SystemInfo for UnthrottlePlayer {
		type Dependencies = ();
	}
}

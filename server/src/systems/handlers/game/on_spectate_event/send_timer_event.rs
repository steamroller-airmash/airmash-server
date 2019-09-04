use specs::prelude::*;

use crate::component::channel::OnTimerEvent;
use crate::component::event::{PlayerSpectate, TimerEvent};
use crate::component::time::*;
use crate::consts::timer::SCORE_BOARD;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct SendTimerEvent;

#[derive(SystemData, EventDeps)]
pub struct SendTimerEventData<'a> {
	timerchannel: Write<'a, OnTimerEvent>,
	thisframe: Read<'a, ThisFrame>,
}

impl EventHandlerTypeProvider for SendTimerEvent {
	type Event = PlayerSpectate;
}

impl<'a> EventHandler<'a> for SendTimerEvent {
	type SystemData = SendTimerEventData<'a>;

	fn on_event(&mut self, evt: &PlayerSpectate, data: &mut Self::SystemData) {
		// No need to inform clients that they are in
		// spec if they are already in spec
		if evt.is_dead || evt.is_spec {
			return;
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

system_info! {
	impl SystemInfo for SendTimerEvent {
		type Dependencies = super::KnownEventSources;
	}
}

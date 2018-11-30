use shrev::*;
use specs::*;

use types::*;

use consts::timer::SCORE_BOARD;
use dispatch::SystemInfo;
use systems::handlers::game::on_player_hit::AllPlayerHitSystems;

use component::event::TimerEvent;
use component::event::*;
use component::time::ThisFrame;

use utils::{EventHandler, EventHandlerTypeProvider};

use protocol::server::PlayerKill;

#[derive(Default)]
pub struct DisplayMessage;

#[derive(SystemData)]
pub struct DisplayMessageData<'a> {
	pub entities: Entities<'a>,
	pub conns: Read<'a, Connections>,
	pub timerevent: Write<'a, EventChannel<TimerEvent>>,
	pub thisframe: Read<'a, ThisFrame>,
}

impl EventHandlerTypeProvider for DisplayMessage {
	type Event = PlayerKilled;
}

impl<'a> EventHandler<'a> for DisplayMessage {
	type SystemData = DisplayMessageData<'a>;

	fn on_event(&mut self, evt: &PlayerKilled, data: &mut Self::SystemData) {
		let packet = PlayerKill {
			id: evt.player.into(),
			killer: Some(evt.killer.into()),
			pos: evt.pos,
		};

		if evt.player == evt.killer {
			warn!("Player {:?} killed themselves!", evt.player);
		}

		data.conns.send_to_visible(evt.pos, packet);

		data.timerevent.single_write(TimerEvent {
			ty: *SCORE_BOARD,
			instant: data.thisframe.0,
			..Default::default()
		});
	}
}

impl SystemInfo for DisplayMessage {
	type Dependencies = (AllPlayerHitSystems);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

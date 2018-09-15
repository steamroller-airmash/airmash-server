use shrev::*;
use specs::*;

use types::*;

use consts::timer::SCORE_BOARD;
use dispatch::SystemInfo;
use systems::handlers::game::on_player_hit::AllPlayerHitSystems;

use component::channel::*;
use component::event::TimerEvent;
use component::time::ThisFrame;

use protocol::server::PlayerKill;

pub struct DisplayMessage {
	reader: Option<OnPlayerKilledReader>,
}

#[derive(SystemData)]
pub struct DisplayMessageData<'a> {
	pub entities: Entities<'a>,
	pub channel: Read<'a, OnPlayerKilled>,
	pub conns: Read<'a, Connections>,
	pub timerevent: Write<'a, EventChannel<TimerEvent>>,
	pub thisframe: Read<'a, ThisFrame>,
}

impl DisplayMessage {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for DisplayMessage {
	type SystemData = DisplayMessageData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerKilled>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let packet = PlayerKill {
				id: evt.player.into(),
				killer: Some(evt.killer.into()),
				pos: evt.pos,
			};

			if evt.player == evt.killer {
				warn!("Player {:?} killed themselves!", evt.player);
			}

			data.conns.send_to_all(packet);

			data.timerevent.single_write(TimerEvent {
				ty: *SCORE_BOARD,
				instant: data.thisframe.0,
				..Default::default()
			});
		}
	}
}

impl SystemInfo for DisplayMessage {
	type Dependencies = (AllPlayerHitSystems);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

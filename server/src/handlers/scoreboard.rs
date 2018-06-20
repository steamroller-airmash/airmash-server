use shrev::*;
use specs::*;
use types::*;

use component::event::ScoreBoardTimerEvent;
use component::flag::IsPlayer;

use airmash_protocol::server::{ScoreBoard, ScoreBoardData, ScoreBoardRanking};
use airmash_protocol::{to_bytes, ServerPacket};
use std::vec::Vec;
use websocket::OwnedMessage;

pub struct ScoreBoardTimerHandler {
	reader: Option<ReaderId<ScoreBoardTimerEvent>>,
}

impl ScoreBoardTimerHandler {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

#[derive(SystemData)]
pub struct ScoreBoardSystemData<'a> {
	channel: Read<'a, EventChannel<ScoreBoardTimerEvent>>,
	conns: Read<'a, Connections>,

	entities: Entities<'a>,
	scores: ReadStorage<'a, Score>,
	levels: ReadStorage<'a, Level>,
	pos: ReadStorage<'a, Position>,
	flag: ReadStorage<'a, IsPlayer>,
}

impl<'a> System<'a> for ScoreBoardTimerHandler {
	type SystemData = ScoreBoardSystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		self.reader = Some(
			res.fetch_mut::<EventChannel<ScoreBoardTimerEvent>>()
				.register_reader(),
		);

		Self::SystemData::setup(res);
	}

	fn run(&mut self, data: Self::SystemData) {
		if let Some(ref mut reader) = self.reader {
			for _ in data.channel.read(reader) {
				let mut packet_data = (&*data.entities, &data.scores, &data.levels, &data.flag)
					.join()
					.map(|(ent, score, level, _)| ScoreBoardData {
						id: ent,
						score: *score,
						level: *level,
					})
					.collect::<Vec<ScoreBoardData>>();

				packet_data.sort_by(|a, b| a.score.cmp(&b.score));

				let packet_data = packet_data
					.into_iter()
					.take(10)
					.collect::<Vec<ScoreBoardData>>();

				let rankings = (&*data.entities, &data.pos, &data.flag)
					.join()
					.map(|(ent, pos, _)| ScoreBoardRanking { id: ent, pos: *pos })
					.collect::<Vec<ScoreBoardRanking>>();

				let score_board = ScoreBoard {
					data: packet_data,
					rankings: rankings,
				};

				data.conns.send_to_all(OwnedMessage::Binary(
					to_bytes(&ServerPacket::ScoreBoard(score_board)).unwrap(),
				));
			}
		}
	}
}

use dispatch::SystemInfo;
use std::any::Any;
use systems::TimerHandler;
impl SystemInfo for ScoreBoardTimerHandler {
	type Dependencies = (TimerHandler);

	fn new(_: Box<Any>) -> Self {
		Self::new()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

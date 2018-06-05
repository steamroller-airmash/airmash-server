use shrev::*;
use specs::*;
use types::*;
use component::event::ScoreBoardTimerEvent;

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
				let mut packet_data = (&*data.entities, &data.scores, &data.levels)
					.join()
					.map(|(ent, score, level)| ScoreBoardData {
						id: ent.id() as u16,
						score: score.0,
						level: level.0,
					})
					.collect::<Vec<ScoreBoardData>>();

				packet_data.sort_by(|a, b| a.score.cmp(&b.score));

				let packet_data = packet_data
					.into_iter()
					.take(10)
					.collect::<Vec<ScoreBoardData>>();

				let rankings = (&*data.entities, &data.pos)
					.join()
					.map(|(ent, pos)| ScoreBoardRanking {
						id: ent.id() as u16,
						x: ((pos.x.inner() / 16384.0) as i32 + 128) as u8,
						y: ((pos.y.inner() / 8192.0) as i32 + 128) as u8,
					})
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

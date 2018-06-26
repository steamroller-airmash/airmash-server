use specs::*;
use types::*;

use consts::timer::SCORE_BOARD;

use component::channel::{OnTimerEvent, OnTimerEventReader};
use component::flag::{IsPlayer, IsSpectating, IsDead};

use protocol::server::{ScoreBoard, ScoreBoardData, ScoreBoardRanking};
use protocol::{to_bytes, ServerPacket};
use OwnedMessage;

lazy_static! {
	static ref SPEC_POSITION: Position =
		Position::new(Distance::new(-16384.0), Distance::new(-8192.0));
}

pub struct ScoreBoardTimerHandler {
	reader: Option<OnTimerEventReader>,
}

impl ScoreBoardTimerHandler {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

#[derive(SystemData)]
pub struct ScoreBoardSystemData<'a> {
	channel: Read<'a, OnTimerEvent>,
	conns: Read<'a, Connections>,

	entities: Entities<'a>,
	scores: ReadStorage<'a, Score>,
	levels: ReadStorage<'a, Level>,
	pos: ReadStorage<'a, Position>,
	flag: ReadStorage<'a, IsPlayer>,
	isspec: ReadStorage<'a, IsSpectating>,
	isdead: ReadStorage<'a, IsDead>,
}

impl<'a> System<'a> for ScoreBoardTimerHandler {
	type SystemData = ScoreBoardSystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		self.reader = Some(res.fetch_mut::<OnTimerEvent>().register_reader());

		Self::SystemData::setup(res);
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if evt.ty != *SCORE_BOARD {
				continue;
			}

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
				.map(|(ent, pos, _)| {
					if data.isspec.get(ent).is_some() || data.isdead.get(ent).is_some() {
						(ent, *SPEC_POSITION)
					} else {
						(ent, *pos)
					}
				})
				.map(|(ent, pos)| ScoreBoardRanking { id: ent, pos: pos })
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

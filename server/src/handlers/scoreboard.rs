use specs::*;
use types::*;

use consts::timer::SCORE_BOARD;

use component::channel::{OnTimerEvent, OnTimerEventReader};
use component::flag::{IsDead, IsPlayer, IsSpectating};
use component::time::JoinTime;

use protocol::server::{ScoreBoard, ScoreBoardData, ScoreBoardRanking};

use std::cmp::{Ordering, Reverse};

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
	join_time: ReadStorage<'a, JoinTime>,
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

			let mut packet_data = (
				&*data.entities,
				&data.scores,
				&data.levels,
				&data.flag,
				&data.join_time,
			).join()
				.map(|(ent, score, level, _, join_time)| {
					(
						ScoreBoardData {
							id: ent.into(),
							score: *score,
							level: *level,
						},
						join_time.0,
					)
				})
				.collect::<Vec<_>>();

			packet_data.sort_by(|a, b| {
				let ord = Reverse(a.0.score).cmp(&Reverse(b.0.score));

				match ord {
					Ordering::Equal => a.1.cmp(&b.1),
					_ => ord,
				}
			});

			let packet_data = packet_data
				.into_iter()
				.take(10)
				.map(|(s, _)| s)
				.collect::<Vec<_>>();

			let rankings = (&*data.entities, &data.pos, &data.flag)
				.join()
				.map(|(ent, pos, _)| {
					if data.isspec.get(ent).is_some() || data.isdead.get(ent).is_some() {
						(ent, None)
					} else {
						(ent, Some(*pos))
					}
				})
				.map(|(ent, pos)| ScoreBoardRanking {
					id: ent.into(),
					pos: pos,
				})
				.collect::<Vec<ScoreBoardRanking>>();

			let score_board = ScoreBoard {
				data: packet_data,
				rankings: rankings,
			};

			data.conns.send_to_all(score_board);
		}
	}
}

use dispatch::SystemInfo;
use systems::TimerHandler;

impl SystemInfo for ScoreBoardTimerHandler {
	type Dependencies = (TimerHandler);

	fn new() -> Self {
		Self::new()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

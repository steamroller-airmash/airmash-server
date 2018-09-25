use specs::*;

use server::component::channel::*;
use server::component::counter::*;
use server::component::flag::*;
use server::protocol::server::{ScoreDetailedCTF, ScoreDetailedCTFEntry};
use server::*;

use component::Captures;

#[derive(Default)]
pub struct ScoreDetailed {
	reader: Option<OnScoreDetailedReader>,
}

#[derive(SystemData)]
pub struct ScoreDetailedData<'a> {
	channel: Read<'a, OnScoreDetailed>,
	conns: Read<'a, Connections>,

	entities: Entities<'a>,
	level: ReadStorage<'a, Level>,
	captures: ReadStorage<'a, Captures>,
	score: ReadStorage<'a, Score>,
	kills: ReadStorage<'a, TotalKills>,
	deaths: ReadStorage<'a, TotalDeaths>,
	ping: ReadStorage<'a, PlayerPing>,
	is_player: ReadStorage<'a, IsPlayer>,
}

impl<'a> System<'a> for ScoreDetailed {
	type SystemData = ScoreDetailedData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnScoreDetailed>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let scores = (
				&*data.entities,
				&data.level,
				&data.captures,
				&data.score,
				&data.kills,
				&data.deaths,
				&data.ping,
				data.is_player.mask(),
			)
				.join()
				.map(|(ent, level, captures, score, kills, deaths, ping, ..)| {
					ScoreDetailedCTFEntry {
						id: ent.into(),
						level: *level,
						captures: captures.0 as u16,
						score: *score,
						kills: kills.0 as u16,
						deaths: deaths.0 as u16,
						// TODO: Track this
						damage: 0.0,
						ping: ping.0 as u16,
					}
				}).collect();

			let packet = ScoreDetailedCTF { scores };

			data.conns.send_to(evt.0, packet);
		}
	}
}

impl SystemInfo for ScoreDetailed {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

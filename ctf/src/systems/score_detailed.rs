use specs::*;

use crate::server::component::counter::*;
use crate::server::component::event::*;
use crate::server::component::flag::*;
use crate::server::protocol::server::{ScoreDetailedCTF, ScoreDetailedCTFEntry};
use crate::server::utils::*;
use crate::server::*;

use crate::component::Captures;

#[derive(Default)]
pub struct ScoreDetailed;

#[derive(SystemData)]
pub struct ScoreDetailedData<'a> {
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

impl EventHandlerTypeProvider for ScoreDetailed {
	type Event = ScoreDetailedEvent;
}

impl<'a> EventHandler<'a> for ScoreDetailed {
	type SystemData = ScoreDetailedData<'a>;

	fn on_event(&mut self, evt: &ScoreDetailedEvent, data: &mut Self::SystemData) {
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
			})
			.collect();

		let packet = ScoreDetailedCTF { scores };

		data.conns.send_to(evt.0, packet);
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

use specs::*;

use types::systemdata::*;
use types::*;

use component::counter::*;
use component::event::*;
use protocol::server::ScoreUpdate;
use SystemInfo;

use utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct UpdateScore;

#[derive(SystemData)]
pub struct UpdateScoreData<'a> {
	entities: Entities<'a>,
	conns: SendToAll<'a>,

	score: WriteStorage<'a, Score>,
	upgrades: WriteStorage<'a, Upgrades>,

	earnings: WriteStorage<'a, Earnings>,
	total_kills: WriteStorage<'a, TotalKills>,
	total_deaths: WriteStorage<'a, TotalDeaths>,
}

impl UpdateScore {
	fn send_update<'a>(player: Entity, data: &UpdateScoreData<'a>) {
		let score = *try_get!(player, data.score);
		let earnings = try_get!(player, data.earnings).0;
		let upgrades = try_get!(player, data.upgrades);
		let total_kills = try_get!(player, data.total_kills).0;
		let total_deaths = try_get!(player, data.total_deaths).0;

		// FIXME: This probably doesn't need to be sent to all
		//        players. I think we could get away with only
		//        sending it to visible player or even just
		//        the player that the update is about.
		data.conns.send_to_all(ScoreUpdate {
			id: player.into(),
			score,
			earnings,
			upgrades: upgrades.unused,
			total_deaths,
			total_kills,
		});
	}
}

impl EventHandlerTypeProvider for UpdateScore {
	type Event = PlayerKilled;
}

impl<'a> EventHandler<'a> for UpdateScore {
	type SystemData = UpdateScoreData<'a>;

	fn on_event(&mut self, evt: &PlayerKilled, data: &mut Self::SystemData) {
		// Don't do anything if either of the players have left
		if !data.entities.is_alive(evt.player) {
			return;
		}
		if !data.entities.is_alive(evt.killer) {
			return;
		}

		{
			let kills = try_get!(evt.killer, mut data.total_kills);
			let deaths = try_get!(evt.player, mut data.total_deaths);

			//FIXME: Figure out proper bounty transfer formula
			let transfer = (try_get!(evt.player, mut data.score).0 + 3) / 4;

			try_get!(evt.player, mut data.score).0 -= transfer;
			try_get!(evt.killer, mut data.score).0 += transfer + 25;

			kills.0 += 1;
			deaths.0 += 1;
		}

		Self::send_update(evt.player, &data);
		Self::send_update(evt.killer, &data);
	}
}

impl SystemInfo for UpdateScore {
	type Dependencies = (super::DisplayMessage);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

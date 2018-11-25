use specs::*;
use types::*;

use dispatch::SystemInfo;

use component::channel::*;
use component::counter::*;
use component::event::*;
use component::time::ThisFrame;

use protocol::server::ScoreUpdate;

use utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct UpdateScore;

#[derive(SystemData)]
pub struct UpdateScoreData<'a> {
	pub entities: Entities<'a>,
	pub channel: Read<'a, OnPlayerKilled>,
	pub conns: Read<'a, Connections>,
	pub thisframe: Read<'a, ThisFrame>,

	pub score: WriteStorage<'a, Score>,
	pub powerups: WriteStorage<'a, Powerups>,
	pub upgrades: WriteStorage<'a, Upgrades>,

	pub earnings: WriteStorage<'a, Earnings>,
	pub total_kills: WriteStorage<'a, TotalKills>,
	pub total_deaths: WriteStorage<'a, TotalDeaths>,
}

impl UpdateScore {
	fn send_update<'a>(player: Entity, data: &UpdateScoreData<'a>) {
		let score = *try_get!(player, data.score);
		let earnings = try_get!(player, data.earnings).0;
		let upgrades = try_get!(player, data.upgrades);
		let total_kills = try_get!(player, data.total_kills).0;
		let total_deaths = try_get!(player, data.total_deaths).0;

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

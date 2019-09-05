use specs::prelude::*;

use super::*;

use crate::component::counter::*;
use crate::component::event::*;
use crate::types::systemdata::Connections;
use crate::types::*;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

use crate::protocol::server::ScoreUpdate;

#[derive(Default)]
pub struct SendScoreUpdate;

#[derive(SystemDataCustom)]
pub struct SendScoreUpdateData<'a> {
	conns: Connections<'a>,

	score: ReadStorage<'a, Score>,
	earnings: ReadStorage<'a, Earnings>,
	upgrades: ReadStorage<'a, Upgrades>,
	total_kills: ReadStorage<'a, TotalKills>,
	total_deaths: ReadStorage<'a, TotalDeaths>,
}

impl EventHandlerTypeProvider for SendScoreUpdate {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for SendScoreUpdate {
	type SystemData = SendScoreUpdateData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		let score = try_get!(evt.id, data.score);
		let earnings = try_get!(evt.id, data.earnings);
		let upgrades = try_get!(evt.id, data.upgrades);
		let total_kills = try_get!(evt.id, data.total_kills);
		let total_deaths = try_get!(evt.id, data.total_deaths);

		let packet = ScoreUpdate {
			id: evt.id.into(),
			score: *score,
			earnings: earnings.0,
			upgrades: upgrades.unused,
			total_kills: total_kills.0,
			total_deaths: total_deaths.0,
		};

		data.conns.send_to_all(packet);
	}
}

system_info! {
	impl SystemInfo for SendScoreUpdate {
		type Dependencies = (
			SendLogin,
			InitConnection,
		);
	}
}

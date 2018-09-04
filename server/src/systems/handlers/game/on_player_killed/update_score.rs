use shrev::*;
use specs::*;

use types::*;

use dispatch::SystemInfo;

use component::channel::*;
use component::counter::*;
use component::event::TimerEvent;
use component::time::ThisFrame;

use protocol::server::ScoreUpdate;

pub struct UpdateScore {
	reader: Option<OnPlayerKilledReader>,
}

#[derive(SystemData)]
pub struct UpdateScoreData<'a> {
	pub entities: Entities<'a>,
	pub channel: Read<'a, OnPlayerKilled>,
	pub conns: Read<'a, Connections>,
	pub timerevent: Write<'a, EventChannel<TimerEvent>>,
	pub thisframe: Read<'a, ThisFrame>,

	pub score: WriteStorage<'a, Score>,
	pub powerups: WriteStorage<'a, Powerups>,
	pub upgrades: WriteStorage<'a, Upgrades>,

	pub earnings: WriteStorage<'a, Earnings>,
	pub total_kills: WriteStorage<'a, TotalKills>,
	pub total_deaths: WriteStorage<'a, TotalDeaths>,
}

impl UpdateScore {
	pub fn new() -> Self {
		Self { reader: None }
	}

	fn send_update<'a>(player: Entity, data: &UpdateScoreData<'a>) {
		let score = *data.score.get(player).unwrap();
		let earnings = data.earnings.get(player).unwrap().0;
		let upgrades = data.upgrades.get(player).unwrap();
		let total_kills = data.total_kills.get(player).unwrap().0;
		let total_deaths = data.total_deaths.get(player).unwrap().0;

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

impl<'a> System<'a> for UpdateScore {
	type SystemData = UpdateScoreData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerKilled>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			// Don't do anything if either of the players have left
			if !data.entities.is_alive(evt.player) {
				continue;
			}
			if !data.entities.is_alive(evt.killer) {
				continue;
			}

			let transfer = (data.score.get(evt.player).unwrap().0 + 3) / 4;

			data.score.get_mut(evt.player).unwrap().0 -= transfer;
			data.score.get_mut(evt.killer).unwrap().0 += transfer + 25;

			data.total_kills.get_mut(evt.killer).unwrap().0 += 1;
			data.total_deaths.get_mut(evt.player).unwrap().0 += 1;

			Self::send_update(evt.player, &data);
			Self::send_update(evt.killer, &data);
		}
	}
}

impl SystemInfo for UpdateScore {
	type Dependencies = (super::DisplayMessage);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

use server::*;
use specs::*;

use component::*;
use config as ctfconfig;

use server::component::counter::*;
use server::protocol::server::ScoreUpdate;
use server::types::*;

pub struct UpdateScore {
	reader: Option<OnFlagReader>,
}

#[derive(SystemData)]
pub struct UpdateScoreData<'a> {
	pub channel: Read<'a, OnFlag>,
	pub conns: Read<'a, Connections>,
	pub players_game: Read<'a, PlayersGame>,

	pub scores: WriteStorage<'a, Score>,
	pub earnings: WriteStorage<'a, Earnings>,
	pub kills: ReadStorage<'a, TotalKills>,
	pub deaths: ReadStorage<'a, TotalDeaths>,
	pub upgrades: ReadStorage<'a, Upgrades>,
}

impl UpdateScore {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for UpdateScore {
	type SystemData = UpdateScoreData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnFlag>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			match evt.ty {
				FlagEventType::Capture => (),
				_ => continue,
			};

			let player = evt.player.unwrap();
			let players_game = data.players_game.0;
			let score_increase = ctfconfig::FLAG_CAP_BOUNTY_BASE.0 * players_game.min(10);

			let ref mut earnings = data.earnings.get_mut(player).unwrap().0;
			let score = data.scores.get_mut(player).unwrap();

			score.0 += score_increase;
			earnings.0 += score_increase;

			let packet = ScoreUpdate {
				id: player.into(),
				score: *score,
				earnings: *earnings,

				total_kills: data.kills.get(player).unwrap().0,
				total_deaths: data.deaths.get(player).unwrap().0,

				upgrades: data.upgrades.get(player).unwrap().unused,
			};

			data.conns.send_to_all(packet);
		}
	}
}

use systems::PickupFlagSystem;

impl SystemInfo for UpdateScore {
	type Dependencies = PickupFlagSystem;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

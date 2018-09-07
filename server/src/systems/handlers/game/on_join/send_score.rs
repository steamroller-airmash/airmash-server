use specs::*;
use types::*;

use super::*;

use SystemInfo;

use component::channel::*;
use component::counter::*;

use protocol::server::ScoreUpdate;

pub struct SendScoreUpdate {
	reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct SendScoreUpdateData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,
	pub conns: Read<'a, Connections>,

	pub score: ReadStorage<'a, Score>,
	pub earnings: ReadStorage<'a, Earnings>,
	pub upgrades: ReadStorage<'a, Upgrades>,
	pub total_kills: ReadStorage<'a, TotalKills>,
	pub total_deaths: ReadStorage<'a, TotalDeaths>,
}

impl<'a> System<'a> for SendScoreUpdate {
	type SystemData = SendScoreUpdateData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			channel,
			conns,

			score,
			earnings,
			upgrades,
			total_kills,
			total_deaths,
		} = data;

		for evt in channel.read(self.reader.as_mut().unwrap()) {
			let score = score.get(evt.id).unwrap();
			let earnings = earnings.get(evt.id).unwrap();
			let upgrades = upgrades.get(evt.id).unwrap();
			let total_kills = total_kills.get(evt.id).unwrap();
			let total_deaths = total_deaths.get(evt.id).unwrap();

			let packet = ScoreUpdate {
				id: evt.id.into(),
				score: *score,
				earnings: earnings.0,
				upgrades: upgrades.unused,
				total_kills: total_kills.0,
				total_deaths: total_deaths.0,
			};

			conns.send_to_all(packet);
		}
	}
}

impl SystemInfo for SendScoreUpdate {
	type Dependencies = (
		InitTraits,
		InitEarnings,
		InitKillCounters,
		SendLogin,
		InitConnection,
		InitState,
	);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

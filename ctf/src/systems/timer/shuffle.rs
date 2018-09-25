use specs::*;

use server::component::channel::*;
use server::component::counter::*;
use server::component::flag::*;
use server::*;

use server::protocol::server::{PlayerReteam, PlayerReteamPlayer};

use component::*;
use consts::*;
use shuffle::*;

#[derive(Default)]
pub struct Shuffle {
	reader: Option<OnTimerEventReader>,
}

#[derive(SystemData)]
pub struct ShuffleData<'a> {
	channel: Read<'a, OnTimerEvent>,
	shuffler: ReadExpect<'a, Box<ShuffleProvider + Sync + Send>>,
	conns: Read<'a, Connections>,
	entities: Entities<'a>,

	is_player: ReadStorage<'a, IsPlayer>,
	captures: ReadStorage<'a, Captures>,
	score: ReadStorage<'a, Score>,
	team: WriteStorage<'a, Team>,
	kills: ReadStorage<'a, TotalKills>,
	deaths: ReadStorage<'a, TotalDeaths>,
}

impl<'a> System<'a> for Shuffle {
	type SystemData = ShuffleData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnTimerEvent>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if evt.ty != *RETEAM_TIMER {
				continue;
			}

			let player_info = (
				&*data.entities,
				&data.team,
				&data.score,
				&data.captures,
				&data.kills,
				&data.deaths,
				data.is_player.mask(),
			)
				.join()
				.map(
					|(ent, team, score, captures, kills, deaths, ..)| PlayerShuffleInfo {
						player: ent,
						team: *team,
						score: *score,
						captures: captures.0,
						kills: kills.0,
						deaths: deaths.0,
					},
				).collect::<Vec<_>>();

			let swaps = data.shuffler.shuffle(player_info);

			for swap in swaps.iter() {
				*data.team.get_mut(swap.player).unwrap() = swap.new_team;
			}

			let swaps = swaps
				.into_iter()
				.map(|swap| PlayerReteamPlayer {
					id: swap.player.into(),
					team: swap.new_team,
				}).collect::<Vec<_>>();

			let packet = PlayerReteam { players: swaps };

			data.conns.send_to_all(packet);
		}
	}
}

impl SystemInfo for Shuffle {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

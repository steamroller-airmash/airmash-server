use specs::*;

use server::component::counter::*;
use server::component::event::*;
use server::component::flag::*;
use server::types::GameModeWriter;
use server::utils::*;
use server::*;

use server::protocol::server::{PlayerReteam, PlayerReteamPlayer};

use component::*;
use config::*;
use consts::*;
use gamemode::CTFGameMode;
use shuffle::*;

#[derive(Default)]
pub struct Shuffle;

#[derive(SystemData)]
pub struct ShuffleData<'a> {
	shuffler: ReadExpect<'a, Box<ShuffleProvider + Sync + Send>>,
	conns: Read<'a, Connections>,
	entities: Entities<'a>,
	gamemode: GameModeWriter<'a, CTFGameMode>,

	is_player: ReadStorage<'a, IsPlayer>,
	captures: ReadStorage<'a, Captures>,
	score: ReadStorage<'a, Score>,
	teams: WriteStorage<'a, Team>,
	kills: ReadStorage<'a, TotalKills>,
	deaths: ReadStorage<'a, TotalDeaths>,
}

impl EventHandlerTypeProvider for Shuffle {
	type Event = TimerEvent;
}

impl<'a> EventHandler<'a> for Shuffle {
	type SystemData = ShuffleData<'a>;

	fn on_event(&mut self, evt: &TimerEvent, data: &mut Self::SystemData) {
		if evt.ty != *RETEAM_TIMER {
			return;
		}

		let player_info = (
			&*data.entities,
			&data.teams,
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
			)
			.collect::<Vec<_>>();

		let swaps = data.shuffler.shuffle(player_info);

		for swap in swaps.iter() {
			data.teams.insert(swap.player, swap.new_team).unwrap();
		}

		let (red, blue) = swaps
			.iter()
			.map(|x| {
				if x.new_team == RED_TEAM {
					(1, 0)
				} else {
					(0, 1)
				}
			})
			.fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

		let gamemode: &mut CTFGameMode = &mut *data.gamemode;

		gamemode.redteam = red;
		gamemode.blueteam = blue;

		let swaps = swaps
			.into_iter()
			.map(|swap| PlayerReteamPlayer {
				id: swap.player.into(),
				team: swap.new_team,
			})
			.collect::<Vec<_>>();

		let packet = PlayerReteam { players: swaps };

		data.conns.send_to_all(packet);
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

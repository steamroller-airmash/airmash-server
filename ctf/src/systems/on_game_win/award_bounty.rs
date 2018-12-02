use specs::*;

use server::component::channel::*;
use server::component::counter::*;
use server::component::event::*;
use server::component::flag::IsPlayer;
use server::component::time::*;
use server::consts::timer::SCORE_BOARD;
use server::types::Upgrades;
use server::utils::*;
use server::*;

use server::protocol::server::ScoreUpdate;

use component::*;
use config::GAME_WIN_BOUNTY_BASE;
use systems::on_flag::CheckWin;

/// Award bounty to all members of the winning team
#[derive(Default)]
pub struct AwardBounty;

#[derive(SystemData)]
pub struct AwardBountyData<'a> {
	players_game: Read<'a, PlayersGame>,
	timer_channel: Write<'a, OnTimerEvent>,
	this_frame: Read<'a, ThisFrame>,
	conns: Read<'a, Connections>,

	entities: Entities<'a>,
	is_player: ReadStorage<'a, IsPlayer>,
	team: ReadStorage<'a, Team>,
	score: WriteStorage<'a, Score>,
	earnings: WriteStorage<'a, Earnings>,
	kills: ReadStorage<'a, TotalKills>,
	deaths: ReadStorage<'a, TotalDeaths>,
	upgrades: ReadStorage<'a, Upgrades>,
}

impl EventHandlerTypeProvider for AwardBounty {
	type Event = GameWinEvent;
}

impl<'a> EventHandler<'a> for AwardBounty {
	type SystemData = AwardBountyData<'a>;

	fn on_event(&mut self, evt: &GameWinEvent, data: &mut Self::SystemData) {
		let ref conns = data.conns;

		let team = evt.winning_team;
		let bounty = data.players_game.0.min(10) * GAME_WIN_BOUNTY_BASE.0;

		(
			&data.team,
			&mut data.score,
			&*data.entities,
			&mut data.earnings,
			&data.kills,
			&data.deaths,
			&data.upgrades,
			data.is_player.mask(),
		)
			.join()
			.filter(|(player_team, ..)| team == **player_team)
			.for_each(
				|(_, score, player, earnings, kills, deaths, upgrades, ..)| {
					score.0 += bounty;
					(earnings.0).0 += bounty;

					let packet = ScoreUpdate {
						id: player.into(),
						score: *score,
						earnings: earnings.0,
						total_deaths: deaths.0,
						total_kills: kills.0,
						upgrades: upgrades.unused,
					};

					conns.send_to_player(player, packet)
				},
			);

		data.timer_channel.single_write(TimerEvent {
			ty: *SCORE_BOARD,
			instant: data.this_frame.0,
			data: None,
		});
	}
}

impl SystemInfo for AwardBounty {
	type Dependencies = CheckWin;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

use server::*;
use specs::*;

use component::*;
use config as ctfconfig;

use server::component::counter::*;
use server::protocol::server::ScoreUpdate;
use server::types::*;
use server::utils::*;

#[derive(Default)]
pub struct UpdateScore;

#[derive(SystemData)]
pub struct UpdateScoreData<'a> {
	conns: Read<'a, Connections>,
	players_game: Read<'a, PlayersGame>,

	scores: WriteStorage<'a, Score>,
	earnings: WriteStorage<'a, Earnings>,
	kills: ReadStorage<'a, TotalKills>,
	deaths: ReadStorage<'a, TotalDeaths>,
	upgrades: ReadStorage<'a, Upgrades>,
}

impl EventHandlerTypeProvider for UpdateScore {
	type Event = FlagEvent;
}

impl<'a> EventHandler<'a> for UpdateScore {
	type SystemData = UpdateScoreData<'a>;

	fn on_event(&mut self, evt: &FlagEvent, data: &mut Self::SystemData) {
		match evt.ty {
			FlagEventType::Capture => (),
			_ => return,
		};

		let player = evt.player.unwrap();
		let players_game = data.players_game.0;
		let score_increase = ctfconfig::FLAG_CAP_BOUNTY_BASE.0 * players_game.min(10);

		let ref mut earnings = try_get!(player, mut data.earnings).0;
		let score = try_get!(player, mut data.scores);

		score.0 += score_increase;
		earnings.0 += score_increase;

		let packet = ScoreUpdate {
			id: player.into(),
			score: *score,
			earnings: *earnings,

			total_kills: try_get!(player, data.kills).0,
			total_deaths: try_get!(player, data.deaths).0,

			upgrades: try_get!(player, data.upgrades).unused,
		};

		data.conns.send_to_all(packet);
	}
}

use systems::PickupFlagSystem;

impl SystemInfo for UpdateScore {
	type Dependencies = PickupFlagSystem;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

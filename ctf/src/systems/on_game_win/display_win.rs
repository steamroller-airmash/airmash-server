use server::*;
use specs::*;

use component::*;
use config::*;
use systems::on_flag::CheckWin;

use server::component::counter::PlayersGame;
use server::protocol::server::ServerCustom;
use server::protocol::ServerCustomType;

#[derive(Default)]
pub struct DisplayWin {
	reader: Option<OnGameWinReader>,
}

#[derive(SystemData)]
pub struct DisplayWinData<'a> {
	channel: Read<'a, OnGameWin>,
	conns: Read<'a, Connections>,
	players_game: Read<'a, PlayersGame>,
}

impl<'a> System<'a> for DisplayWin {
	type SystemData = DisplayWinData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnGameWin>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			// TODO: Use serde and define a struct for this
			// it's not a huge issue here but this too messy
			let text = format!(
				"{{\"w\":{},\"b\":{},\"t\":{}}}",
				evt.winning_team.0,
				data.players_game.0.min(10) * GAME_WIN_BOUNTY_BASE.0,
				13, // seconds
			);

			let packet = ServerCustom {
				ty: ServerCustomType::CTFWin,
				data: text,
			};

			data.conns.send_to_all(packet)
		}
	}
}

impl SystemInfo for DisplayWin {
	type Dependencies = CheckWin;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

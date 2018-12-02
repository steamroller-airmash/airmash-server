use server::*;
use specs::*;

use component::*;
use config::*;
use systems::on_flag::CheckWin;

use server::component::counter::PlayersGame;
use server::protocol::server::ServerCustom;
use server::protocol::ServerCustomType;
use server::utils::*;

#[derive(Default)]
pub struct DisplayWin;

#[derive(SystemData)]
pub struct DisplayWinData<'a> {
	conns: Read<'a, Connections>,
	players_game: Read<'a, PlayersGame>,
}

impl EventHandlerTypeProvider for DisplayWin {
	type Event = GameWinEvent;
}

impl<'a> EventHandler<'a> for DisplayWin {
	type SystemData = DisplayWinData<'a>;

	fn on_event(&mut self, evt: &GameWinEvent, data: &mut Self::SystemData) {
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

impl SystemInfo for DisplayWin {
	type Dependencies = CheckWin;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

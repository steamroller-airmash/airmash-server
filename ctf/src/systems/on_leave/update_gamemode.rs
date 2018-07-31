use specs::*;

use server::component::channel::*;
use server::component::event::PlayerLeave;
use server::systems::handlers::packet::OnCloseHandler;
use server::*;

use CTFGameMode;
use BLUE_TEAM;
use RED_TEAM;

pub struct UpdateGameModeOnPlayerLeave {
	reader: Option<OnPlayerLeaveReader>,
}

#[derive(SystemData)]
pub struct UpdateGameModeOnPlayerLeaveData<'a> {
	pub gamemode: GameModeWriter<'a, CTFGameMode>,
	pub channel: Read<'a, OnPlayerLeave>,

	pub teams: ReadStorage<'a, Team>,
}

impl<'a> System<'a> for UpdateGameModeOnPlayerLeave {
	type SystemData = UpdateGameModeOnPlayerLeaveData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerLeave>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for PlayerLeave(ent) in data.channel.read(self.reader.as_mut().unwrap()) {
			let team = data.teams.get(*ent).unwrap();

			if *team == RED_TEAM {
				data.gamemode.redteam -= 1;
			} else if *team == BLUE_TEAM {
				data.gamemode.blueteam -= 1;
			} else {
				unimplemented!();
			}
		}
	}
}

impl SystemInfo for UpdateGameModeOnPlayerLeave {
	type Dependencies = OnCloseHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

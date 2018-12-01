use specs::*;

use SystemInfo;

use systems::handlers::packet::LoginHandler;

use component::counter::*;
use component::event::*;
use utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct InitKillCounters;

#[derive(SystemData)]
pub struct InitKillCountersData<'a> {
	total_kills: WriteStorage<'a, TotalKills>,
	total_deaths: WriteStorage<'a, TotalDeaths>,
}

impl EventHandlerTypeProvider for InitKillCounters {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for InitKillCounters {
	type SystemData = InitKillCountersData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		data.total_kills.insert(evt.id, TotalKills(0)).unwrap();
		data.total_deaths.insert(evt.id, TotalDeaths(0)).unwrap();
	}
}

impl SystemInfo for InitKillCounters {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

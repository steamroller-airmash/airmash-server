use specs::*;

use std::time::Duration;

use consts::timer::RESPAWN_TIME;
use types::*;

use SystemInfo;

use component::channel::*;
use component::event::TimerEvent;
use component::flag::IsDead;
use component::time::ThisFrame;

use protocol::server::MobDespawnCoords;

pub struct PlayerKilledCleanup {
	reader: Option<OnPlayerKilledReader>,
}

#[derive(SystemData)]
pub struct PlayerKilledCleanupData<'a> {
	pub entities: Entities<'a>,
	pub channel: Read<'a, OnPlayerKilled>,
	pub conns: Read<'a, Connections>,
	pub thisframe: Read<'a, ThisFrame>,
	pub timerchannel: Write<'a, OnTimerEvent>,

	pub name: ReadStorage<'a, Name>,
	pub level: ReadStorage<'a, Level>,
	pub isdead: WriteStorage<'a, IsDead>,
	pub mob: ReadStorage<'a, Mob>,

	pub futdispatch: ReadExpect<'a, FutureDispatcher>,
}

impl PlayerKilledCleanup {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for PlayerKilledCleanup {
	type SystemData = PlayerKilledCleanupData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerKilled>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			data.isdead.insert(evt.player, IsDead).unwrap();

			let despawn_packet = MobDespawnCoords {
				id: evt.missile.into(),
				ty: *data.mob.get(evt.missile).unwrap(),
				pos: evt.pos,
			};

			data.conns.send_to_all(despawn_packet);

			let player = evt.player;

			// Set a timer event to make the player respawn
			data.futdispatch
				.run_delayed(Duration::from_secs(2), move |instant| {
					Some(TimerEvent {
						ty: *RESPAWN_TIME,
						instant,
						data: Some(Box::new(player)),
					})
				});
		}
	}
}

impl SystemInfo for PlayerKilledCleanup {
	type Dependencies = super::on_player_hit::InflictDamage;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

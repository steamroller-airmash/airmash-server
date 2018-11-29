use specs::*;

use types::*;

use SystemInfo;

use component::channel::*;
use component::flag::IsDead;

use protocol::server::MobDespawnCoords;

pub struct PlayerKilledCleanup {
	reader: Option<OnPlayerKilledReader>,
}

#[derive(SystemData)]
pub struct PlayerKilledCleanupData<'a> {
	channel: Read<'a, OnPlayerKilled>,
	conns: Read<'a, Connections>,

	isdead: WriteStorage<'a, IsDead>,
	mob: ReadStorage<'a, Mob>,
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
				ty: *try_get!(evt.missile, data.mob),
				pos: evt.pos,
			};

			data.conns.send_to_visible(evt.pos, despawn_packet);
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

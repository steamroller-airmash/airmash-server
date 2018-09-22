use specs::*;

use types::*;

use component::channel::*;
use component::event::*;
use component::flag::*;
use consts::timer::*;

use systems::TimerHandler;
use SystemInfo;

pub struct PlayerRespawnSystem {
	reader: Option<OnTimerEventReader>,
}

#[derive(SystemData)]
pub struct PlayerRespawnSystemData<'a> {
	pub channel: Read<'a, OnTimerEvent>,
	pub conns: Read<'a, Connections>,
	pub respawn_channel: Write<'a, OnPlayerRespawn>,

	pub team: ReadStorage<'a, Team>,

	pub pos: WriteStorage<'a, Position>,
	pub vel: WriteStorage<'a, Velocity>,
	pub rot: WriteStorage<'a, Rotation>,
	pub health: WriteStorage<'a, Health>,
	pub energy: WriteStorage<'a, Energy>,

	pub is_dead: WriteStorage<'a, IsDead>,
	pub is_spec: ReadStorage<'a, IsSpectating>,

	pub gamemode: GameModeWriter<'a, GameMode>,
}

impl<'a> System<'a> for PlayerRespawnSystem {
	type SystemData = PlayerRespawnSystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnTimerEvent>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if evt.ty != *RESPAWN_TIME {
				continue;
			}

			let player =
				match evt.data {
					Some(ref dat) => match (*dat).downcast_ref::<Entity>() {
						Some(val) => *val,
						None => {
							error!("Unable to downcast TimerEvent data to Entity! Event will be skipped.");
							continue;
						}
					},
					None => continue,
				};

			data.respawn_channel.single_write(PlayerRespawn {
				player,
				prev_status: PlayerRespawnPrevStatus::Dead,
			});
		}
	}
}

impl SystemInfo for PlayerRespawnSystem {
	type Dependencies = TimerHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

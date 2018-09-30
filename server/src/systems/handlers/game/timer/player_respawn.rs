use specs::*;

use component::channel::*;
use component::event::*;
use consts::timer::*;

use systems::TimerHandler;
use SystemInfo;

pub struct PlayerRespawnSystem {
	reader: Option<OnTimerEventReader>,
}

#[derive(SystemData)]
pub struct PlayerRespawnSystemData<'a> {
	channel: Read<'a, OnTimerEvent>,
	respawn_channel: Write<'a, OnPlayerRespawn>,
	entities: Entities<'a>,
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

			if !data.entities.is_alive(player) {
				continue;
			}

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

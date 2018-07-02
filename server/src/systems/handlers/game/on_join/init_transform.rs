use specs::*;
use types::*;

use GameMode;
use GameModeWriter;
use SystemInfo;

use component::channel::*;

pub struct InitTransform {
	reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct InitTransformData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,
	pub gamemode: GameModeWriter<'a, GameMode>,

	pub pos: WriteStorage<'a, Position>,
	pub rot: WriteStorage<'a, Rotation>,
	pub vel: WriteStorage<'a, Velocity>,
	pub team: ReadStorage<'a, Team>,
}

impl<'a> System<'a> for InitTransform {
	type SystemData = InitTransformData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			channel,
			mut gamemode,

			mut pos,
			mut rot,
			mut vel,
			team,
		} = data;

		for evt in channel.read(self.reader.as_mut().unwrap()) {
			let player_pos = gamemode
				.get_mut()
				.spawn_pos(evt.0, *team.get(evt.0).unwrap());

			pos.insert(evt.0, player_pos).unwrap();
			rot.insert(evt.0, Rotation::default()).unwrap();
			vel.insert(evt.0, Velocity::default()).unwrap();
		}
	}
}

impl SystemInfo for InitTransform {
	type Dependencies = super::InitTraits;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

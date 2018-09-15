use server::*;
use specs::*;

use server::component::channel::*;
use server::component::time::ThisFrame;
use server::protocol::server::GameFlag;
use server::protocol::FlagUpdateType;

use component::*;

pub struct Drop {
	reader: Option<OnPlayerLeaveReader>,
}

#[derive(SystemData)]
pub struct DropData<'a> {
	pub entities: Entities<'a>,
	pub channel: Read<'a, OnPlayerLeave>,
	pub conns: Read<'a, Connections>,
	pub pos: WriteStorage<'a, Position>,
	pub is_flag: ReadStorage<'a, IsFlag>,
	pub carrier: WriteStorage<'a, FlagCarrier>,
	pub teams: ReadStorage<'a, Team>,
	pub lastdrop: WriteStorage<'a, LastDrop>,
	pub thisframe: Read<'a, ThisFrame>,
}

impl Drop {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for Drop {
	type SystemData = DropData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerLeave>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			channel,
			conns,
			mut pos,
			is_flag,
			teams,
			mut carrier,
			entities,
			mut lastdrop,
			thisframe,
		} = data;

		for evt in channel.read(self.reader.as_mut().unwrap()) {
			let player_pos = *pos.get(evt.0).unwrap();

			(&mut pos, &mut carrier, &is_flag, &*entities, &mut lastdrop)
				.join()
				.filter(|(_, carrier, _, _, _)| carrier.0.is_some() && carrier.0.unwrap() == evt.0)
				.for_each(|(pos, carrier, _, ent, lastdrop)| {
					let team = teams.get(ent).unwrap();

					let packet = GameFlag {
						ty: FlagUpdateType::Position,
						flag: Flag(*team),
						id: None,
						pos: player_pos,
						blueteam: 0,
						redteam: 0,
					};

					*pos = player_pos;
					*carrier = FlagCarrier(None);
					*lastdrop = LastDrop {
						// None doesn't do what we want, so pick an entity
						// that we won't see again. (i.e. the player that
						// is leaving). This also prevents the player from
						// picking the flag up again if the pickup update
						// runs after this system
						player: Some(ent),
						time: thisframe.0,
					};

					conns.send_to_all(packet);
				});
		}
	}
}

use server::systems::PositionUpdate;

impl SystemInfo for Drop {
	type Dependencies = PositionUpdate;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

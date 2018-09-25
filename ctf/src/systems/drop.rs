use server::*;
use specs::*;

use server::component::channel::{OnCommand, OnCommandReader};
use server::component::time::ThisFrame;
use server::protocol::server::GameFlag;
use server::protocol::FlagUpdateType;

use component::*;

pub struct DropSystem {
	reader: Option<OnCommandReader>,
}

#[derive(SystemData)]
pub struct DropSystemData<'a> {
	pub channel: Read<'a, OnCommand>,
	pub conns: Read<'a, Connections>,
	pub thisframe: Read<'a, ThisFrame>,

	pub entities: Entities<'a>,
	pub pos: WriteStorage<'a, Position>,
	pub team: ReadStorage<'a, Team>,
	pub is_flag: ReadStorage<'a, IsFlag>,
	pub carrier: WriteStorage<'a, FlagCarrier>,
	pub lastdrop: WriteStorage<'a, LastDrop>,
	pub flagchannel: Write<'a, OnFlag>,
}

impl DropSystem {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for DropSystem {
	type SystemData = DropSystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnCommand>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			channel,
			conns,
			thisframe,
			entities,
			mut pos,
			team,
			is_flag,
			mut carrier,
			mut lastdrop,
			mut flagchannel,
		} = data;

		for evt in channel.read(self.reader.as_mut().unwrap()) {
			if evt.1.com != "drop" {
				continue;
			}

			let player = match conns.0.get(&evt.0) {
				Some(conn) => match conn.player {
					Some(p) => p,
					None => continue,
				},
				None => continue,
			};

			let p_pos = *pos.get(player).unwrap();

			(
				&mut pos,
				&team,
				&is_flag,
				&mut carrier,
				&mut lastdrop,
				&*entities,
			)
				.join()
				.filter(|(_, _, _, carrier, _, _)| {
					carrier.0.is_some() && carrier.0.unwrap() == player
				}).for_each(|(fpos, team, _, carrier, lastdrop, ent)| {
					let packet = GameFlag {
						ty: FlagUpdateType::Position,
						flag: Flag(*team),
						id: None,
						pos: p_pos,
						blueteam: 0,
						redteam: 0,
					};

					flagchannel.single_write(FlagEvent {
						ty: FlagEventType::Drop,
						player: carrier.0,
						flag: ent,
					});

					*fpos = p_pos;
					*carrier = FlagCarrier(None);
					*lastdrop = LastDrop {
						player: Some(player),
						time: thisframe.0,
					};

					conns.send_to_all(packet);
				});
		}
	}
}

use super::PickupFlagSystem;

impl SystemInfo for DropSystem {
	type Dependencies = PickupFlagSystem;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

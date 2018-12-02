use server::*;
use specs::*;

use server::component::event::*;
use server::component::time::ThisFrame;
use server::protocol::server::GameFlag;
use server::protocol::FlagUpdateType;
use server::utils::*;

use component::*;

#[derive(Default)]
pub struct Drop;

#[derive(SystemData)]
pub struct DropData<'a> {
	entities: Entities<'a>,
	conns: Read<'a, Connections>,
	pos: WriteStorage<'a, Position>,
	is_flag: ReadStorage<'a, IsFlag>,
	carrier: WriteStorage<'a, FlagCarrier>,
	teams: ReadStorage<'a, Team>,
	lastdrop: WriteStorage<'a, LastDrop>,
	thisframe: Read<'a, ThisFrame>,
}

impl EventHandlerTypeProvider for Drop {
	type Event = PlayerLeave;
}

impl<'a> EventHandler<'a> for Drop {
	type SystemData = DropData<'a>;

	fn on_event(&mut self, evt: &PlayerLeave, data: &mut Self::SystemData) {
		let thisframe = *data.thisframe;
		let ref conns = data.conns;

		let player_pos = *try_get!(evt.0, data.pos);

		(
			&mut data.pos,
			&mut data.carrier,
			&data.is_flag,
			&data.entities,
			&mut data.lastdrop,
			&data.teams,
		)
			.join()
			.filter(|(_, carrier, _, _, _, _)| carrier.0.is_some() && carrier.0.unwrap() == evt.0)
			.for_each(|(pos, carrier, _, ent, lastdrop, team)| {
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

use server::systems::PositionUpdate;

impl SystemInfo for Drop {
	type Dependencies = PositionUpdate;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

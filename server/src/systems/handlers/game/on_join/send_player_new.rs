use specs::*;
use types::*;

use SystemInfo;

use component::event::*;
use protocol::server::PlayerNew;
use protocol::Upgrades as ProtocolUpgrades;
use utils::{EventHandler, EventHandlerTypeProvider};

/// Send a `PlayerNew` packet to all other players when
/// a new player joins.
#[derive(Default)]
pub struct SendPlayerNew;

#[derive(SystemData)]
pub struct SendPlayerNewData<'a> {
	conns: Read<'a, Connections>,

	pos: ReadStorage<'a, Position>,
	rot: ReadStorage<'a, Rotation>,
	plane: ReadStorage<'a, Plane>,
	team: ReadStorage<'a, Team>,
	status: ReadStorage<'a, Status>,
	flag: ReadStorage<'a, FlagCode>,
	upgrades: ReadStorage<'a, Upgrades>,
	powerups: ReadStorage<'a, Powerups>,
	name: ReadStorage<'a, Name>,
}

impl EventHandlerTypeProvider for SendPlayerNew {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for SendPlayerNew {
	type SystemData = SendPlayerNewData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		let powerups = data.powerups.get(evt.id);

		let upgrades = ProtocolUpgrades {
			speed: try_get!(evt.id, data.upgrades).speed,
			inferno: powerups.inferno(),
			shield: powerups.shield(),
		};

		let player_new = PlayerNew {
			id: evt.id.into(),
			status: *try_get!(evt.id, data.status),
			name: try_get!(evt.id, data.name).0.clone(),
			ty: *try_get!(evt.id, data.plane),
			team: *try_get!(evt.id, data.team),
			pos: *try_get!(evt.id, data.pos),
			rot: *try_get!(evt.id, data.rot),
			flag: *try_get!(evt.id, data.flag),
			upgrades,
		};

		data.conns.send_to_others(evt.id, player_new);
	}
}

impl SystemInfo for SendPlayerNew {
	type Dependencies = (
		super::InitTraits,
		super::InitConnection,
		super::InitState,
		super::InitTransform,
		super::SendPlayerPowerup,
		super::SendLogin,
	);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

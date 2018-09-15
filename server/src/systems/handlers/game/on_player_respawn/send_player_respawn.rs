use specs::*;

use component::channel::*;
use component::flag::*;
use types::*;
use SystemInfo;

use protocol::server::PlayerRespawn;
use protocol::Upgrades as ProtocolUpgrades;

use systems::handlers::command::AllCommandHandlers;
use systems::handlers::game::on_join::AllJoinHandlers;
use systems::handlers::game::on_player_respawn::SetTraits;

/// Send a [`PlayerRespawn`] packet to
/// all visible players if the target
/// player is not currently spectating.
#[derive(Default)]
pub struct SendPlayerRespawn {
	reader: Option<OnPlayerRespawnReader>,
}

#[derive(SystemData)]
pub struct SendPlayerRespawnData<'a> {
	channel: Read<'a, OnPlayerRespawn>,
	conns: Read<'a, Connections>,

	is_spec: ReadStorage<'a, IsSpectating>,
	pos: ReadStorage<'a, Position>,
	rot: ReadStorage<'a, Rotation>,
}

impl<'a> System<'a> for SendPlayerRespawn {
	type SystemData = SendPlayerRespawnData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerRespawn>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if data.is_spec.get(evt.player).is_some() {
				continue;
			}

			let player = evt.player;

			data.conns.send_to_visible(
				player,
				PlayerRespawn {
					id: player.into(),
					pos: *data.pos.get(player).unwrap(),
					rot: *data.rot.get(player).unwrap(),
					upgrades: ProtocolUpgrades::default(),
				},
			);
		}
	}
}

impl SystemInfo for SendPlayerRespawn {
	type Dependencies = (AllJoinHandlers, SetTraits, AllCommandHandlers);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

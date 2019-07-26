use crate::types::systemdata::*;
use crate::types::*;
use specs::*;

use crate::GameMode;
use crate::GameModeWriter;
use crate::SystemInfo;

use crate::component::event::*;
use crate::protocol::server::{Login, LoginPlayer};
use crate::protocol::Upgrades as ProtocolUpgrades;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct SendLogin;

#[derive(SystemData)]
pub struct SendLoginData<'a> {
	conns: SendToPlayer<'a>,
	entities: Entities<'a>,
	gamemode: GameModeWriter<'a, dyn GameMode>,
	clock: ReadClock<'a>,

	pos: ReadStorage<'a, Position>,
	rot: ReadStorage<'a, Rotation>,
	plane: ReadStorage<'a, Plane>,
	team: ReadStorage<'a, Team>,
	status: ReadStorage<'a, Status>,
	flag: ReadStorage<'a, FlagCode>,
	upgrades: ReadStorage<'a, Upgrades>,
	powerups: ReadStorage<'a, Powerups>,
	name: ReadStorage<'a, Name>,
	level: ReadStorage<'a, Level>,
}

impl SendLogin {
	fn get_player_data<'a>(data: &SendLoginData<'a>) -> Vec<LoginPlayer> {
		(
			&*data.entities,
			&data.pos,
			&data.rot,
			&data.plane,
			&data.name,
			&data.flag,
			&data.upgrades,
			&data.level,
			&data.status,
			&data.team,
		)
			.join()
			.map({
				|(ent, pos, rot, plane, name, flag, upgrades, level, status, team)| {
					let powerups = data.powerups.get(ent);
					let upgrade_field = ProtocolUpgrades {
						speed: upgrades.speed,
						shield: powerups.shield(),
						inferno: powerups.inferno(),
					};

					LoginPlayer {
						id: ent.into(),
						status: *status,
						level: *level,
						name: name.0.clone(),
						ty: *plane,
						team: *team,
						pos: *pos,
						rot: *rot,
						flag: *flag,
						upgrades: upgrade_field,
					}
				}
			})
			.collect()
	}
}

impl EventHandlerTypeProvider for SendLogin {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for SendLogin {
	type SystemData = SendLoginData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		let player_data = Self::get_player_data(&data);

		let gamemode = data.gamemode.get();

		let packet = Login {
			clock: data.clock.get(),
			id: evt.id.into(),
			room: gamemode.room(),
			success: true,
			token: "none".to_owned(),
			team: *try_get!(evt.id, data.team),
			ty: gamemode.gametype(),
			players: player_data,
		};

		data.conns.send_to_player(evt.id, packet);
	}
}

impl SystemInfo for SendLogin {
	type Dependencies = (super::InitTraits, super::InitConnection, super::InitState);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

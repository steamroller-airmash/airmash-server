use specs::*;
use types::*;

use std::convert::TryFrom;
use std::time::Duration;

use component::channel::*;
use component::event::*;
use component::flag::*;
use component::time::*;

use protocol::server::{Error, PlayerType};
use protocol::ErrorType;

use utils::{EventHandler, EventHandlerTypeProvider};

use systems::handlers::game::on_join::InitTraits;
use systems::PacketHandler;
use SystemInfo;

#[derive(Default)]
pub struct Respawn;

#[derive(SystemData)]
pub struct RespawnData<'a> {
	health: WriteStorage<'a, Health>,
	planes: WriteStorage<'a, Plane>,
	is_spec: WriteStorage<'a, IsSpectating>,
	is_dead: WriteStorage<'a, IsDead>,
	last_key: ReadStorage<'a, LastKeyTime>,

	conns: Read<'a, Connections>,
	channel: Write<'a, OnPlayerRespawn>,
	this_frame: Read<'a, ThisFrame>,
}

impl EventHandlerTypeProvider for Respawn {
	type Event = CommandEvent;
}

impl<'a> EventHandler<'a> for Respawn {
	type SystemData = RespawnData<'a>;

	fn on_event(&mut self, evt: &CommandEvent, data: &mut Self::SystemData) {
		let &(conn, ref packet) = evt;

		let player = match data.conns.associated_player(conn) {
			Some(p) => p,
			None => return,
		};

		if packet.com != "respawn" {
			return;
		}

		let plane = match parse_plane(&packet.data) {
			Ok(p) => p,
			Err(_) => return,
		};

		let allowed = !check_allowed(
			data.is_spec.get(player).is_some(),
			data.is_dead.get(player).is_some(),
			data.health.get(player).unwrap(),
			data.last_key.get(player).unwrap(),
			&*data.this_frame,
		);

		if !allowed {
			data.conns.send_to(
				conn,
				Error {
					error: ErrorType::IdleRequiredBeforeRespawn,
				},
			);

			return;
		}

		let prev_status =
			match data.is_spec.get(player).is_some() || data.is_dead.get(player).is_some() {
				true => PlayerRespawnPrevStatus::Dead,
				false => PlayerRespawnPrevStatus::Alive,
			};

		data.planes.insert(player, plane).unwrap();
		data.is_spec.remove(player);
		// Prevent updates from happening until the actual respawn
		// process is finished.
		data.is_dead.insert(player, IsDead).unwrap();

		data.channel.single_write(PlayerRespawn {
			player,
			prev_status,
		});

		data.conns.send_to_all(PlayerType {
			id: player.into(),
			ty: plane,
		});
	}
}

impl SystemInfo for Respawn {
	type Dependencies = (PacketHandler, InitTraits);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

fn check_allowed(
	is_spec: bool,
	is_dead: bool,
	health: &Health,
	last_key: &LastKeyTime,
	this_frame: &ThisFrame,
) -> bool {
	// A player may not respawn during the 2s cooldown
	// period after dying (the is represented by the
	// IsDead flag)
	!is_dead
		|| (
		// If the player is spectating then they may respawn
		// at any time
		is_spec || (
			// Players that don't have full health may not respawn
			!(*health < Health::new(1.0))
			// Players that have pressed a key within the last
			// 2 seconds may not respawn
			&& !(this_frame.0 - last_key.0 < Duration::from_secs(2))
		)
	)
}

fn parse_plane<'a>(s: &'a str) -> Result<Plane, ()> {
	let num: u32 = s.parse().map_err(|_| {})?;
	Plane::try_from(num).map_err(|_| {})
}

#[cfg(test)]
mod test {
	use super::*;
	use types::Plane::*;
	#[test]
	fn parse_valid_plane() {
		let planes = vec![
			("1", Predator),
			("2", Goliath),
			("3", Mohawk),
			("4", Tornado),
			("5", Prowler),
		];

		for (s, ty) in planes {
			match parse_plane(s) {
				Ok(v) => assert_eq!(v, ty),
				Err(_) => panic!("Parsing a valid string \"{}\" failed!", s),
			}
		}
	}

	#[test]
	fn parse_out_of_range_plane() {
		assert!(parse_plane("256").is_err());
	}

	#[test]
	fn parse_empty_plane() {
		assert!(parse_plane("").is_err());
	}

	#[test]
	fn parse_non_number_plane() {
		assert!(parse_plane("foo").is_err());
	}
}

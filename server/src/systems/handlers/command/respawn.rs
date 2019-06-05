use specs::*;

use types::systemdata::*;
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

	conns: SendToAll<'a>,
	channel: Write<'a, OnPlayerRespawn>,
	this_frame: Read<'a, ThisFrame>,

	pub velocity: ReadStorage<'a, Velocity>,
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

		let allowed = check_allowed(
			data.is_dead.get(player).is_some(),
			data.is_spec.get(player).is_some(),
			data.health.get(player).unwrap(),
			data.last_key.get(player).unwrap(),
			data.velocity.get(player).unwrap(),
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
	velocity: &Vector2<Speed>,
	this_frame: &ThisFrame,
) -> bool {
	// Note to my future self and maintainers:
	//  Originally this code was written as one big
	//  boolean expression. This was unclear and caused
	//  some bugs so now it's been rewritten in this
	//  fashion. This is a lot more clear and I'd prefer
	//  if it stayed that way.

	// A player may not respawn during the 2s cooldown
	// period after dying (this is represented by the
	// IsDead flag)
	if is_dead {
		println!("respawn denied - 2s cooldown after death");
		return false;
	}

	// If the player is spectating then they may respawn
	// at any time. Note that is_dead will prevent respawning
	// during the first 2 seconds after going into spec.
	if is_spec {
		println!("respawn allowed - is speccing");
		return true;
	}

	let smin = Speed::new(-0.1);
	let smax = Speed::new(0.1);
	if !(smin < velocity.x && smax > velocity.x) {
		println!("respawn denied - xvel too high X {} {}", velocity.x, velocity.y);
		return false;
	}

	if !(smin < velocity.y && smax > velocity.y) {
		println!("respawn denied - yvel too high {} {}", velocity.x, velocity.y);
		return false;
	}

	// Players that don't have full health may not respawn
	if *health < Health::new(1.0) {
		println!("respawn denied - poor health");
		return false;
	}

	// Players that have not pressed a key within the last
	// 2 seconds may not respawn.
	if (this_frame.0 - last_key.0) < Duration::from_secs(2) {
		println!("respawn denied - pressed key too recently");
		return false;
	}

	true
}

fn parse_plane<'a>(s: &'a str) -> Result<Plane, ()> {
	let num: u32 = s.parse().map_err(|_| {})?;
	Plane::try_from(num).map_err(|_| {})
}

#[cfg(test)]
mod test {
	use super::*;
	use std::time::*;
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

	#[test]
	fn check_not_allowed_dead() {
		assert!(!check_allowed(
			true,
			true,
			&Health::new(1.0),
			&LastKeyTime(Instant::now() - Duration::from_secs(4)),
			&ThisFrame(Instant::now())
		));
	}

	#[test]
	fn check_allowed_spec() {
		assert!(check_allowed(
			true,
			false,
			&Health::new(1.0),
			&LastKeyTime(Instant::now() - Duration::from_secs(5)),
			&ThisFrame(Instant::now())
		));
	}

	#[test]
	fn check_not_allowed_low_health() {
		assert!(!check_allowed(
			false,
			false,
			&Health::new(0.5),
			&LastKeyTime(Instant::now() - Duration::from_secs(10)),
			&ThisFrame(Instant::now())
		));
	}
}

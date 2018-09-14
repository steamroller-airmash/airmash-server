use specs::*;
use types::*;

use std::convert::TryFrom;

use component::channel::*;
use component::event::*;
use component::flag::*;

use protocol::server::PlayerType;

use utils::{EventHandler, EventHandlerTypeProvider};

use systems::PacketHandler;
use SystemInfo;

#[derive(Default)]
pub struct Respawn;

#[derive(SystemData)]
pub struct RespawnData<'a> {
	health: WriteStorage<'a, Health>,
	planes: WriteStorage<'a, Plane>,
	is_spec: WriteStorage<'a, IsSpectating>,

	conns: Read<'a, Connections>,
	channel: Write<'a, OnPlayerRespawn>,
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

		// Make sure player health is a 100% before allowing respawn
		if *data.health.get(player).unwrap() < Health::new(1.0) {
			return;
		}

		data.planes.insert(player, plane).unwrap();
		data.is_spec.remove(player);

		data.channel.single_write(PlayerRespawn { player });

		data.conns.send_to_all(PlayerType {
			id: player.into(),
			ty: plane,
		});
	}
}

impl SystemInfo for Respawn {
	type Dependencies = PacketHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
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

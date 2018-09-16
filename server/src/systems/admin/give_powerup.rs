use specs::*;
use types::*;

use utils::event_handler::{EventHandler, EventHandlerTypeProvider};
use SystemInfo;

use component::channel::OnPlayerPowerup;
use component::event::{CommandEvent, PlayerPowerup};
use component::flag::IsPlayer;
use protocol::server::CommandReply;
use protocol::{CommandReplyType, PowerupType};
use systems::PacketHandler;

use std::convert::TryFrom;
use std::option::NoneError;

use serde_json;

#[derive(Debug, Serialize)]
enum CommandParseError<'a> {
	NoSuchPlayer(u16),
	InvalidPlayerId(&'a str),
	InvalidPowerupType(u8),
	InvalidPowerupName(&'a str),
	MissingArguments,
}

impl<'a> From<NoneError> for CommandParseError<'a> {
	fn from(_: NoneError) -> Self {
		CommandParseError::MissingArguments
	}
}

fn collect_fixed<I, T>(mut iter: I) -> Option<[T; 2]>
where
	I: Iterator<Item = T>,
{
	Some([iter.next()?, iter.next()?])
}

fn parse_powerup<'a>(ident: &'a str) -> Result<PowerupType, CommandParseError<'a>> {
	let res: Result<u8, _> = ident.parse();
	match res {
		Ok(u) => match PowerupType::try_from(u) {
			Ok(p) => Ok(p),
			Err(_) => Err(CommandParseError::InvalidPowerupType(u)),
		},
		Err(_) => Ok(match ident {
			"inferno" => PowerupType::Inferno,
			"shield" => PowerupType::Shield,
			_ => return Err(CommandParseError::InvalidPowerupName(ident)),
		}),
	}
}

fn parse_id<'a>(ident: &'a str) -> Result<u16, CommandParseError<'a>> {
	ident
		.parse()
		.map_err(|_| CommandParseError::InvalidPlayerId(ident))
}

fn parse_command_iter<'a, I>(iter: I) -> Result<(PowerupType, u16), CommandParseError<'a>>
where
	I: Iterator<Item = &'a str>,
{
	let vals = collect_fixed(iter)?;

	Ok((parse_powerup(vals[0])?, parse_id(vals[1])?))
}

#[derive(Default)]
pub struct GivePowerup;

#[derive(SystemData)]
pub struct GivePowerupData<'a> {
	entities: Entities<'a>,
	channel: Write<'a, OnPlayerPowerup>,
	config: Read<'a, Config>,
	conns: Read<'a, Connections>,
	is_player: ReadStorage<'a, IsPlayer>,
}

impl GivePowerup {
	fn do_command<'a, 'b>(
		evt: &'b CommandEvent,
		data: &mut GivePowerupData<'a>,
	) -> Result<(), CommandParseError<'b>> {
		use self::PowerupType::*;

		let &(conn, ref packet) = evt;

		if !data.config.admin_enabled {
			return Ok(());
		}

		if packet.com != "give-powerup" {
			return Ok(());
		}

		let source = match data.conns.associated_player(conn) {
			Some(p) => p,
			None => return Ok(()),
		};

		let (ty, id) = parse_command_iter(packet.data.split(" "))?;
		let player = if id != 0 {
			data.entities.entity(id as u32)
		} else {
			source
		};

		if !data.entities.is_alive(player) {
			return Err(CommandParseError::NoSuchPlayer(id));
		}

		if !data.is_player.get(player).is_none() {
			return Err(CommandParseError::NoSuchPlayer(id));
		}

		let duration = match ty {
			Shield => data.config.shield_duration,
			Inferno => data.config.inferno_duration,
		};

		data.channel.single_write(PlayerPowerup {
			player,
			duration,
			ty,
		});

		Ok(())
	}
}

impl EventHandlerTypeProvider for GivePowerup {
	type Event = CommandEvent;
}

impl<'a> EventHandler<'a> for GivePowerup {
	type SystemData = GivePowerupData<'a>;

	fn on_event(&mut self, evt: &CommandEvent, data: &mut Self::SystemData) {
		if let Err(e) = Self::do_command(evt, data) {
			let (conn, _) = evt;
			data.conns.send_to(
				*conn,
				CommandReply {
					ty: CommandReplyType::ShowInPopup,
					text: format!("{}", serde_json::to_string_pretty(&e).unwrap()),
				},
			);
		}
	}
}

impl SystemInfo for GivePowerup {
	type Dependencies = PacketHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_inferno_success() {
		let terms: Vec<&str> = vec!["inferno", "1"];

		let (ty, id) = parse_command_iter(terms.into_iter()).expect("Command failed to parse");

		assert_eq!(ty, PowerupType::Inferno);
		assert_eq!(id, 1);
	}

	#[test]
	fn test_shield_success() {
		let terms: Vec<&str> = vec!["shield", "230"];

		let (ty, id) = parse_command_iter(terms.into_iter()).expect("Command failed to parse");

		assert_eq!(ty, PowerupType::Shield);
		assert_eq!(id, 230);
	}

	#[test]
	fn test_invalid_powerup_type_ident() {
		let terms: Vec<&str> = vec!["bork5", "32"];

		let res = parse_command_iter(terms.into_iter());

		assert!(res.is_err());

		if let Err(e) = res {
			match e {
				CommandParseError::InvalidPowerupName(name) => assert_eq!(name, "bork5"),
				_ => panic!("Incorrect error returned {:?}", e),
			}
		}
	}

	#[test]
	fn test_invalid_powerup_type_num() {
		let terms: Vec<&str> = vec!["255", "8988"];

		let res = parse_command_iter(terms.into_iter());

		if let Err(e) = res {
			match e {
				CommandParseError::InvalidPowerupType(v) => assert_eq!(v, 255),
				_ => panic!("Incorrect error returned {:?}", e),
			}
		} else {
			panic!("Parsing should not have succeeded. Result: {:?}", res);
		}
	}
}

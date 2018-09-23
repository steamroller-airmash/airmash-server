use specs::*;
use types::*;

use std::time::Duration;

use SystemInfo;

use component::channel::*;
use component::event::PlayerSpectate;
use component::event::*;
use component::flag::{IsDead, IsPlayer, IsSpectating};
use component::reference::PlayerRef;
use component::time::{LastKeyTime, ThisFrame};

use protocol::server::Error;
use protocol::ErrorType;

use utils::{EventHandler, EventHandlerTypeProvider};

use systems::handlers::game::on_join::InitTraits;
use systems::PacketHandler;

#[derive(Default)]
pub struct Spectate;

#[derive(SystemData)]
pub struct SpectateData<'a> {
	pub conns: Read<'a, Connections>,
	pub channel: Write<'a, OnPlayerSpectate>,
	pub this_frame: Read<'a, ThisFrame>,

	pub is_spec: WriteStorage<'a, IsSpectating>,
	pub is_dead: WriteStorage<'a, IsDead>,
	pub is_player: ReadStorage<'a, IsPlayer>,
	pub spec_target: ReadStorage<'a, PlayerRef>,
	pub entities: Entities<'a>,
	pub health: ReadStorage<'a, Health>,
	pub last_key: ReadStorage<'a, LastKeyTime>,
}

impl EventHandlerTypeProvider for Spectate {
	type Event = CommandEvent;
}

impl<'a> EventHandler<'a> for Spectate {
	type SystemData = SpectateData<'a>;

	fn on_event(&mut self, evt: &CommandEvent, data: &mut SpectateData<'a>) {
		use self::SpectateTarget::*;

		let Self::SystemData {
			conns,
			ref mut channel,
			this_frame,

			is_spec,
			is_dead,
			is_player,
			entities,
			spec_target,
			health,
			last_key,
		} = data;

		let &(conn, ref packet) = evt;

		let player = match conns.associated_player(conn) {
			Some(p) => p,
			None => return,
		};

		if packet.com != "spectate" {
			return;
		}

		let tgt = match parse_spectate_data(&packet.data) {
			Ok(tgt) => tgt,
			Err(_) => return,
		};

		let allowed = !check_allowed(
			is_spec.get(player).is_some(),
			health.get(player).unwrap(),
			last_key.get(player).unwrap(),
			&*this_frame,
		);

		if !allowed {
			conns.send_to(
				conn,
				Error {
					error: ErrorType::IdleRequiredBeforeSpectate,
				},
			);

			return;
		}

		let mut spec_event = PlayerSpectate {
			player: player,
			target: None,
			is_dead: is_dead.get(player).is_some(),
			is_spec: is_spec.get(player).is_some(),
		};

		if is_spec.get(player).is_none() {
			match tgt {
				Next | Prev | Force => {
					spec_event.target = (&**entities, is_player.mask(), !is_spec.mask())
						.join()
						.map(|(ent, ..)| ent)
						.next();
				}
				// A player may not specify the player they wish to
				// spectate when going into spec. This mimics the
				// behaviour of the official server.
				_ => return,
			}
		} else {
			let current = spec_target.get(player).unwrap().0;

			match tgt {
				Next => {
					// Get the next player, wrapping around at the
					// end and defaulting if there is no other player
					let forward = (&**entities, is_player.mask(), !is_spec.mask())
						.join()
						.skip_while(|(ent, ..)| *ent != current)
						.filter(|(ent, ..)| *ent != player)
						.map(|(ent, ..)| ent)
						.next();

					let forward = forward.map(|x| Some(x)).unwrap_or_else(|| {
						(&**entities, is_player.mask(), !is_spec.mask())
							.join()
							.filter(|(ent, ..)| *ent != player)
							.map(|(ent, ..)| ent)
							.next()
					});

					spec_event.target = forward;
				}
				Prev => {
					let back = (&**entities, is_player.mask(), !is_spec.mask())
						.join()
						.take_while(|(ent, ..)| *ent != current)
						.filter(|(ent, ..)| *ent != player)
						.map(|x| x.0)
						.last();

					let back = back.map(|x| Some(x)).unwrap_or_else(|| {
						(&**entities, is_player.mask(), !is_spec.mask())
							.join()
							.filter(|(ent, ..)| *ent != player)
							.map(|x| x.0)
							.last()
					});

					spec_event.target = back;
				}
				Force => {
					// A play is already being spectated, so
					// there is nothing that _needs_ to be done.
					// This behaviour can change at a later time.
				}
				Target(id) => {
					let ent = entities.entity(id);

					// Can't spectate an entity that doesn't exist
					if !entities.is_alive(ent) {
						return;
					}

					// You can't spectate non-players
					if is_player.get(ent).is_none() {
						return;
					}

					spec_event.target = Some(ent);
				}
			}
		}

		channel.single_write(spec_event);
	}
}

impl SystemInfo for Spectate {
	type Dependencies = (PacketHandler, InitTraits);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum SpectateTarget {
	Next,
	Prev,
	Force,
	Target(u32),
}

fn check_allowed(
	is_spec: bool,
	health: &Health,
	last_key: &LastKeyTime,
	this_frame: &ThisFrame,
) -> bool {
	// If the player is already spectating, then they
	// can do the spectate command no matter what
	is_spec
		|| (
		// Players that don't have full health may not spectate
		!(*health < Health::new(1.0))
		// Players that have pressed a key within the last
		// 2 seconds may not spectate
		&& !(this_frame.0 - last_key.0 < Duration::from_secs(2))
	)
}

fn parse_spectate_data<'a>(s: &'a str) -> Result<SpectateTarget, ()> {
	use self::SpectateTarget::*;

	let arg: i32 = s.parse().map_err(|_| ())?;

	// There are no valid values below -3
	if arg < -3 {
		return Err(());
	}

	let tgt = match arg {
		-1 => Next,
		-2 => Prev,
		-3 => Force,
		// All the negative cases have been dealt with, this is safe
		x => Target(x as u32),
	};

	Ok(tgt)
}

#[cfg(test)]
mod test {
	use self::SpectateTarget::*;
	use super::*;

	#[test]
	fn parse_force() {
		assert_eq!(parse_spectate_data("-3"), Ok(Force))
	}

	#[test]
	fn parse_prev() {
		assert_eq!(parse_spectate_data("-2"), Ok(Prev))
	}

	#[test]
	fn parse_next() {
		assert_eq!(parse_spectate_data("-1"), Ok(Next))
	}

	#[test]
	fn parse_id() {
		assert_eq!(parse_spectate_data("5124"), Ok(Target(5124)))
	}

	#[test]
	fn parse_negative_invalid() {
		assert_eq!(parse_spectate_data("-10"), Err(()))
	}

	#[test]
	fn parse_non_number_invalid() {
		assert_eq!(parse_spectate_data("foo"), Err(()))
	}
}

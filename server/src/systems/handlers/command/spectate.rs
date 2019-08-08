use crate::types::*;
use specs::*;

use std::time::Duration;

use crate::component::channel::*;
use crate::component::event::PlayerSpectate;
use crate::component::event::*;
use crate::component::flag::{IsDead, IsPlayer, IsSpectating};
use crate::component::reference::PlayerRef;
use crate::component::time::{LastKeyTime, ThisFrame};

use crate::protocol::server::Error;
use crate::protocol::ErrorType;
use crate::utils::{EventHandler, EventHandlerTypeProvider};
use crate::systems::PacketHandler;

const SPEED_THRESHOLD: f32 = 0.01;
const SPEED_THRESHOLD_SQUARED: f32 = SPEED_THRESHOLD * SPEED_THRESHOLD;

#[derive(Default)]
pub struct Spectate;

#[derive(SystemData)]
pub struct SpectateData<'a> {
	pub conns: Read<'a, Connections>,
	pub channel: Write<'a, OnPlayerSpectate>,
	pub this_frame: Read<'a, ThisFrame>,
	config: Read<'a, Config>,

	pub is_spec: WriteStorage<'a, IsSpectating>,
	pub is_dead: ReadStorage<'a, IsDead>,
	pub is_player: ReadStorage<'a, IsPlayer>,
	pub spec_target: ReadStorage<'a, PlayerRef>,
	pub entities: Entities<'a>,
	pub health: ReadStorage<'a, Health>,
	pub last_key: ReadStorage<'a, LastKeyTime>,
	velocity: ReadStorage<'a, Velocity>,
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
			config,

			is_spec,
			is_dead,
			is_player,
			entities,
			spec_target,
			health,
			last_key,
			velocity,
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

		let mut allowed = check_allowed(
			is_spec.get(player).is_some(),
			try_get!(player, health),
			try_get!(player, last_key),
			&*this_frame,
		);

		let vel = *try_get!(player, velocity);
		let spd2 = vel.length2().inner();
		
		if !config.allow_spectate_while_moving && spd2 >= SPEED_THRESHOLD_SQUARED {
			allowed = false;
		}

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

system_info! {
	impl SystemInfo for Spectate {
		type Dependencies = PacketHandler;
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
	// Note to my future self and maintainers:
	//  Originally this code was written as one big
	//  boolean expression. This was unclear and caused
	//  some bugs so now it's been rewritten in this
	//  fashion. This is a lot more clear and I'd prefer
	//  if it stayed that way.

	// Another note:
	//  This function explicitly doesn't check the velocity
	//  of a player since respawning while moving has always
	//  been possible in airmash. Whether this is a bug in the
	//  original server is debatable but I'd like to stay true
	//  to the original server if possible.

	// If the player is spectating then they may respawn
	// at any time. Note that is_dead will prevent respawning
	// during the first 2 seconds after going into spec.
	if is_spec {
		debug!("spectate allowed, already speccing");
		return true;
	}

	// Players that don't have full health may not respawn
	if *health < Health::new(1.0) {
		debug!("spectate denied, not 100% health");
		return false;
	}

	// Players that have not pressed a key within the last
	// 2 seconds may not respawn.
	if (this_frame.0 - last_key.0) < Duration::from_secs(2) {
		debug!("spectate denied, pressed key too recently");
		return false;
	}

	true
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

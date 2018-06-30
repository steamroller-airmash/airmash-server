use specs::*;

use types::*;

use dispatch::SystemInfo;

use component::channel::*;
use component::reference::PlayerRef;
use component::event::PlayerSpectate;
use component::flag::{IsPlayer, IsSpectating};

use systems::PacketHandler;

pub struct CommandHandler {
	reader: Option<OnCommandReader>,
}

#[derive(SystemData)]
pub struct CommandHandlerData<'a> {
	pub channel: Read<'a, OnCommand>,
	pub conns: Read<'a, Connections>,
	pub specchannel: Write<'a, OnPlayerSpectate>,

	pub is_spec: WriteStorage<'a, IsSpectating>,
	pub is_dead: WriteStorage<'a, IsDead>,
	pub isplayer: ReadStorage<'a, IsPlayer>,
	pub spectarget: ReadStorage<'a, PlayerRef>,
	pub entities: Entities<'a>,
}

impl CommandHandler {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for CommandHandler {
	type SystemData = CommandHandlerData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnCommand>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			channel,
			conns,
			mut specchannel,

			is_spec,
			is_dead,
			isplayer,
			entities,
			spectarget,
		} = data;

		for (id, packet) in channel.read(self.reader.as_mut().unwrap()) {
			if packet.com != "spectate" {
				continue;
			}

			let arg: i32 = match packet.data.parse() {
				Ok(v) => v,
				// Not a valid integer, ignore
				Err(_) => continue,
			};

			let player = match conns.associated_player(*id) {
				Some(p) => p,
				// This packet came from a connection
				// without an associated player, ignore
				None => continue,
			};

			// No valid values below -3, invalid command, ignore
			if arg < -3 {
				continue;
			}

			let mut spec_event = PlayerSpectate {
				player: player,
				target: None,
				is_dead: is_dead.get(player).is_some(),
				is_spec: is_spec.get(player).is_some()
			};

			if is_spec.get(player).is_none() {
				match arg {
					-3...-1 => {
						spec_event.target = (&isplayer, &*entities)
							.join()
							.filter(|(_, ent)| is_spec.get(*ent).is_none())
							.map(|(_, ent)| ent)
							.next();
					}
					// Do nothing if the player didn't pass
					// a value between -1 and -3, other values
					// only apply for players already in spec
					_ => continue,
				}
			} else {
				match arg {
					// Spectate next player
					-1 => {
						let current = spectarget.get(player).unwrap().0;

						// This mess gets the next player
						// including wrapping around and defaulting
						// to the current player if there is no other
						// player
						let forward = (&isplayer, &*entities)
							.join()
							.skip_while(|(_, ent)| *ent != current)
							.filter(|(_, ent)| *ent != player && is_spec.get(*ent).is_none())
							.map(|(_, ent)| ent)
							.next();

						let forward = match forward {
							Some(x) => Some(x),
							None => (&isplayer, &*entities)
								.join()
								.filter(|(_, ent)| *ent != player && is_spec.get(*ent).is_none())
								.map(|(_, ent)| ent)
								.next()
						};
						
						spec_event.target = forward;
					}
					// Spectate prev player
					-2 => {
						let current = spectarget.get(player).unwrap().0;

						let back = (&isplayer, &*entities)
							.join()
							.take_while(|(_, ent)| *ent != current)
							.filter(|(_, ent)| *ent != player && is_spec.get(*ent).is_none())
							.map(|(_, ent)| ent)
							.last();

						let back = match back {
							Some(x) => Some(x),
							None => (&isplayer, &*entities)
								.join()
								.filter(|(_, ent)| *ent != player && is_spec.get(*ent).is_none())
								.map(|(_, ent)| ent)
								.last()
						};

						spec_event.target = back;
					}
					// Force spectate (just pick a player)
					-3 => {
						// We are already spectating a player, so
						// we're good for now. This can be changed
						// at a later time
						continue;
					}
					// Spectate by specific player id
					_ => {
						let ent = entities.entity(arg as u32);

						// Requested an entity that doesn't exist
						if !entities.is_alive(ent) {
							continue;
						}

						// The entity requested was not a player
						if isplayer.get(ent).is_none() {
							continue;
						}

						spec_event.target = Some(ent);
					}
				}
			}

			specchannel.single_write(spec_event);
		}
	}
}

impl SystemInfo for CommandHandler {
	type Dependencies = PacketHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

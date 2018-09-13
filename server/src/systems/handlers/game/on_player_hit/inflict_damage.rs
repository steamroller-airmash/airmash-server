use specs::*;
use types::*;

use dispatch::SystemInfo;

use component::channel::*;
use component::event::{PlayerHit, PlayerKilled};
use component::flag::*;
use component::reference::PlayerRef;

use utils::event_handler::{EventHandler, EventHandlerTypeProvider};

use systems::missile::MissileHit;

#[derive(Default)]
pub struct InflictDamage;

#[derive(SystemData)]
pub struct InflictDamageData<'a> {
	pub entities: Entities<'a>,
	pub channel: Read<'a, OnPlayerHit>,
	pub kill_channel: Write<'a, OnPlayerKilled>,
	pub conns: Read<'a, Connections>,
	pub config: Read<'a, Config>,

	pub health: WriteStorage<'a, Health>,
	pub plane: ReadStorage<'a, Plane>,
	pub upgrades: ReadStorage<'a, Upgrades>,
	pub owner: ReadStorage<'a, PlayerRef>,
	pub player_flag: ReadStorage<'a, IsPlayer>,
	pub powerups: ReadStorage<'a, Powerups>,

	pub mob: ReadStorage<'a, Mob>,
	pub pos: ReadStorage<'a, Position>,
}

impl EventHandlerTypeProvider for InflictDamage {
	type Event = PlayerHit;
}

impl<'a> EventHandler<'a> for InflictDamage {
	type SystemData = InflictDamageData<'a>;

	fn on_event(&mut self, evt: &PlayerHit, data: &mut Self::SystemData) {
		let plane = data.plane.get(evt.player).unwrap();
		let health = data.health.get_mut(evt.player).unwrap();
		let upgrades = data.upgrades.get(evt.player).unwrap();
		let powerups = data.powerups.get(evt.player).unwrap();

		let mob = data.mob.get(evt.missile).unwrap();
		let pos = data.pos.get(evt.missile).unwrap();
		let owner = data.owner.get(evt.missile).unwrap();

		let ref planeconf = data.config.planes[*plane];
		let ref mobconf = data.config.mobs[*mob].missile.unwrap();
		let ref upgconf = data.config.upgrades;

		// No damage can be done if the player is shielded
		if powerups.shield() {
			return;
		}

		*health -= mobconf.damage * planeconf.damage_factor
			/ upgconf.defense.factor[upgrades.defense as usize];

		if health.inner() <= 0.0 {
			data.kill_channel.single_write(PlayerKilled {
				missile: evt.missile,
				player: evt.player,
				killer: owner.0,
				pos: *pos,
			});
		}
	}
}

impl SystemInfo for InflictDamage {
	type Dependencies = MissileHit;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

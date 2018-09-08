use specs::*;
use types::*;

use dispatch::SystemInfo;

use component::channel::*;
use component::event::PlayerKilled;
use component::flag::*;
use component::reference::PlayerRef;

use systems::missile::MissileHit;

pub struct InflictDamage {
	reader: Option<OnPlayerHitReader>,
}

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

impl InflictDamage {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for InflictDamage {
	type SystemData = InflictDamageData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerHit>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
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
			if powerups.shield {
				continue;
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
}

impl SystemInfo for InflictDamage {
	type Dependencies = MissileHit;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

use specs::*;

use SystemInfo;

use types::collision::*;
use types::*;

use component::channel::*;
use component::event::*;
use component::flag::*;

use systems::collision::PlayerUpgradeCollisionSystem;

#[derive(Default)]
pub struct PickupUpgrade {
	reader: Option<OnPlayerUpgradeCollisionReader>,
}

#[derive(SystemData)]
pub struct PickupUpgradeData<'a> {
	channel: Read<'a, OnPlayerUpgradeCollision>,
	upgrade_channel: Write<'a, OnUpgradePickup>,
	entities: Entities<'a>,

	upgrades: WriteStorage<'a, Upgrades>,
	is_player: ReadStorage<'a, IsPlayer>,
}

impl<'a> System<'a> for PickupUpgrade {
	type SystemData = PickupUpgradeData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(
			res.fetch_mut::<OnPlayerUpgradeCollision>()
				.register_reader(),
		);
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let Collision(c1, c2) = evt.0;

			let (player, upgrade) = match data.is_player.get(c1.ent) {
				Some(_) => (c1, c2),
				None => (c2, c1),
			};

			if !data.entities.is_alive(upgrade.ent) {
				continue;
			}

			data.upgrades.get_mut(player.ent).unwrap().unused += 1;

			data.upgrade_channel.single_write(UpgradePickupEvent {
				pos: upgrade.pos,
				upgrade: upgrade.ent,
				player: player.ent,
			})
		}
	}
}

impl SystemInfo for PickupUpgrade {
	type Dependencies = PlayerUpgradeCollisionSystem;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

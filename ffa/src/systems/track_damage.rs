use specs::*;

use components::TotalDamage;

use super::AddDamage;

use airmash_server::component::channel::*;
use airmash_server::component::reference::PlayerRef;
use airmash_server::systems::missile::MissileHit;
use airmash_server::*;

#[derive(Default)]
pub struct TrackDamage {
    reader: Option<OnPlayerHitReader>,
}

#[derive(SystemData)]
pub struct TrackDamageData<'a> {
    entities: Entities<'a>,
    config: Read<'a, Config>,
    channel: Read<'a, OnPlayerHit>,

    mob: ReadStorage<'a, Mob>,
    owner: ReadStorage<'a, PlayerRef>,

    damage: WriteStorage<'a, TotalDamage>,
}

impl<'a> System<'a> for TrackDamage {
    type SystemData = TrackDamageData<'a>;

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.reader = Some(res.fetch_mut::<OnPlayerHit>().register_reader());
    }

    fn run(&mut self, mut data: Self::SystemData) {
        for evt in data.channel.read(self.reader.as_mut().unwrap()) {
            let mob = *data.mob.get(evt.missile).unwrap();
            let owner = *data.owner.get(evt.missile).unwrap();

            if !data.entities.is_alive(owner.0) {
                continue;
            }

            let ref info = data.config.mobs[mob].missile.unwrap();

            data.damage.get_mut(owner.0).unwrap().0 += info.damage * 100.0;
        }
    }
}

impl SystemInfo for TrackDamage {
    type Dependencies = (AddDamage, MissileHit);

    fn name() -> &'static str {
        concat!(module_path!(), "::", line!())
    }

    fn new() -> Self {
        Self::default()
    }
}

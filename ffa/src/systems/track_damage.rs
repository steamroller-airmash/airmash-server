use specs::*;

use crate::components::TotalDamage;

use super::AddDamage;

use airmash_server::component::event::PlayerHit;
use airmash_server::component::flag::IsMissile;
use airmash_server::component::reference::PlayerRef;
use airmash_server::systems::missile::MissileHit;
use airmash_server::utils::{EventHandler, EventHandlerTypeProvider};
use airmash_server::*;

#[derive(Default)]
pub struct TrackDamage;

#[derive(SystemData)]
pub struct TrackDamageData<'a> {
    entities: Entities<'a>,
    config: Read<'a, Config>,

    mob: ReadStorage<'a, Mob>,
    owner: ReadStorage<'a, PlayerRef>,
    is_missile: ReadStorage<'a, IsMissile>,

    damage: WriteStorage<'a, TotalDamage>,
}

impl EventHandlerTypeProvider for TrackDamage {
    type Event = PlayerHit;
}

impl<'a> EventHandler<'a> for TrackDamage {
    type SystemData = TrackDamageData<'a>;

    fn on_event(&mut self, evt: &PlayerHit, data: &mut Self::SystemData) {
        // Ignore invalid missiles
        if !data.is_missile.get(evt.missile).is_some() {
            return;
        }

        let mob = *data.mob.get(evt.missile).unwrap();
        let owner = *data.owner.get(evt.missile).unwrap();

        if !data.entities.is_alive(owner.0) {
            return;
        }

        let ref info = data.config.mobs[mob].missile.unwrap();

        data.damage.get_mut(owner.0).unwrap().0 += info.damage * 100.0;
    }
}

system_info! {
    impl SystemInfo for TrackDamage {
        type Dependencies = (AddDamage, MissileHit);
    }
}

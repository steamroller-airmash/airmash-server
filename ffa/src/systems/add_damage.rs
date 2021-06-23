use specs::*;

use crate::components::TotalDamage;

use airmash_server::component::event::PlayerJoin;
use airmash_server::utils::{EventHandler, EventHandlerTypeProvider};
use airmash_server::*;

#[derive(Default)]
pub struct AddDamage;

#[derive(SystemData)]
pub struct AddDamageData<'a> {
  damage: WriteStorage<'a, TotalDamage>,
  entities: Entities<'a>,
}

impl EventHandlerTypeProvider for AddDamage {
  type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for AddDamage {
  type SystemData = AddDamageData<'a>;

  fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
    if !data.entities.is_alive(evt.id) {
      return;
    }

    data
      .damage
      .insert(evt.id, TotalDamage(Health::new(0.0)))
      .unwrap();
  }
}

system_info! {
    impl SystemInfo for AddDamage {
        type Dependencies = ();
    }
}

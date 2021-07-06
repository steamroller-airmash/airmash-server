use specs::*;

use crate::component::*;

use crate::server::component::event::*;
use crate::server::utils::*;
use crate::server::*;

#[derive(Default)]
pub struct InitCaptures;

#[derive(SystemData)]
pub struct InitCapturesData<'a> {
  entities: Entities<'a>,
  captures: WriteStorage<'a, Captures>,
}

impl EventHandlerTypeProvider for InitCaptures {
  type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for InitCaptures {
  type SystemData = InitCapturesData<'a>;

  fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
    if !data.entities.is_alive(evt.id) {
      return;
    }

    data.captures.insert(evt.id, Captures(0)).unwrap();
  }
}

impl SystemInfo for InitCaptures {
  // It doesn't matter too much when we handle this
  // it can happen the next frame
  type Dependencies = ();

  fn name() -> &'static str {
    concat!(module_path!(), "::", line!())
  }

  fn new() -> Self {
    Self::default()
  }
}

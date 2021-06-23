use crate::SystemInfo;
use specs::*;

use crate::component::event::*;
use crate::component::time::{LastShotTime, ThisFrame};

use crate::utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct SetLastShot;

#[derive(SystemData)]
pub struct SetLastShotData<'a> {
  pub this_frame: Read<'a, ThisFrame>,
  pub last_shot: WriteStorage<'a, LastShotTime>,
}

impl EventHandlerTypeProvider for SetLastShot {
  type Event = MissileFire;
}

impl<'a> EventHandler<'a> for SetLastShot {
  type SystemData = SetLastShotData<'a>;

  fn on_event(&mut self, evt: &MissileFire, data: &mut Self::SystemData) {
    data
      .last_shot
      .insert(evt.player, LastShotTime(data.this_frame.0))
      .unwrap();
  }
}

impl SystemInfo for SetLastShot {
  type Dependencies = super::KnownEventSources;

  fn name() -> &'static str {
    concat!(module_path!(), "::", line!())
  }

  fn new() -> Self {
    Self::default()
  }
}

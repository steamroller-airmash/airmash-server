use specs::*;

use crate::component::stats::FrameCounter;

#[derive(Default)]
pub struct CountFrames;

impl<'a> System<'a> for CountFrames {
  type SystemData = Write<'a, FrameCounter>;

  fn run(&mut self, mut counter: Self::SystemData) {
    counter.0 += 1;
  }
}

system_info! {
  impl SystemInfo for CountFrames {
    type Dependencies = ();
  }
}

use specs::*;

use crate::component::channel::OnTimerEvent;
use crate::component::event::TimerEvent;
use crate::component::time::{LastScoreBoardTime, ThisFrame};

use crate::consts::config::SCORE_BOARD_DURATION;
use crate::consts::timer::*;

#[derive(Default)]
pub struct ScoreBoardTimer;

#[derive(SystemData)]
pub struct ScoreBoardTimerData<'a> {
  last: Write<'a, LastScoreBoardTime>,
  frame: Read<'a, ThisFrame>,
  channel: Write<'a, OnTimerEvent>,
}

impl<'a> System<'a> for ScoreBoardTimer {
  type SystemData = ScoreBoardTimerData<'a>;

  fn run(&mut self, mut data: ScoreBoardTimerData<'a>) {
    let diff = data.frame.0 - data.last.0;
    if diff > SCORE_BOARD_DURATION {
      data.channel.single_write(TimerEvent {
        ty: *SCORE_BOARD,
        instant: data.frame.0,
        ..Default::default()
      });
      data.last.0 = data.frame.0;
    }
  }
}

system_info! {
  impl SystemInfo for ScoreBoardTimer {
    type Dependencies = ();
  }
}

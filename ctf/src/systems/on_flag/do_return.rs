use crate::server::*;
use specs::*;

use crate::config::{BLUE_TEAM, FLAG_HOME_POS, RED_TEAM};

use crate::component::*;

use crate::server::protocol::server::GameFlag;
use crate::server::protocol::FlagUpdateType;
use crate::server::types::systemdata::*;

use crate::server::utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct DoReturn;

#[derive(SystemData)]
pub struct DoReturnData<'a> {
  pos: WriteStorage<'a, Position>,
  flags: ReadExpect<'a, Flags>,

  scores: Read<'a, GameScores>,
  conns: SendToAll<'a>,
  carriers: WriteStorage<'a, FlagCarrier>,
}

impl EventHandlerTypeProvider for DoReturn {
  type Event = FlagEvent;
}

impl<'a> EventHandler<'a> for DoReturn {
  type SystemData = DoReturnData<'a>;

  fn on_event(&mut self, evt: &FlagEvent, data: &mut Self::SystemData) {
    let ref mut pos = data.pos;
    let ref flags = data.flags;
    let ref scores = data.scores;
    let ref conns = data.conns;

    if evt.ty != FlagEventType::Return {
      return;
    }

    let flag_pos = try_get!(evt.flag, mut pos);

    let team;
    if evt.flag == flags.red {
      team = RED_TEAM;
    } else {
      team = BLUE_TEAM;
    }

    let pos = (*FLAG_HOME_POS)[&team];
    *flag_pos = pos;

    try_get!(evt.flag, mut data.carriers).0 = None;

    let packet = GameFlag {
      ty: FlagUpdateType::Position,
      flag: Flag(team),
      id: None,
      pos: *flag_pos,
      blueteam: scores.blueteam,
      redteam: scores.redteam,
    };

    conns.send_to_all(packet);
  }
}

system_info! {
  impl SystemInfo for DoReturn {
    type Dependencies = (
      crate::systems::PickupFlag,
      super::KnownEventSources,
      super::ForceUpdate,
    );
  }
}

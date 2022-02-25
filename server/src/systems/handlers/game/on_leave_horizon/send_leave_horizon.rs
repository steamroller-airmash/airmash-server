use crate::component::event::*;
use crate::protocol::server::EventLeaveHorizon;
use crate::protocol::LeaveHorizonType;
use crate::types::systemdata::*;
use crate::utils::*;
use crate::SystemInfo;

#[derive(Default)]
pub struct SendLeaveHorizon;

#[derive(SystemData)]
pub struct SendLeaveHorizonData<'a> {
  conns: SendToPlayer<'a>,
}

impl EventHandlerTypeProvider for SendLeaveHorizon {
  type Event = LeaveHorizon;
}

impl<'a> EventHandler<'a> for SendLeaveHorizon {
  type SystemData = SendLeaveHorizonData<'a>;

  fn on_event(&mut self, evt: &LeaveHorizon, data: &mut Self::SystemData) {
    use self::LeaveHorizonType::*;

    let ty = match evt.left_ty {
      EntityType::Player => Player,
      _ => Mob,
    };

    data.conns.send_to_player(
      evt.player,
      EventLeaveHorizon {
        id: evt.left.id() as u16,
        ty: ty,
      },
    );
  }
}

impl SystemInfo for SendLeaveHorizon {
  type Dependencies = super::KnownEventSources;

  fn name() -> &'static str {
    concat!(module_path!(), "::", line!())
  }

  fn new() -> Self {
    Self::default()
  }
}

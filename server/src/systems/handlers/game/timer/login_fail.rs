use specs::*;

use crate::component::channel::*;
use crate::consts::timer::*;
use crate::types::*;

use crate::protocol::client::Login;
use crate::protocol::server::Error;
use crate::protocol::ErrorType;

// Login needs write access to just
// about everything
#[derive(SystemData)]
pub struct LoginSystemData<'a> {
  pub conns: Read<'a, Connections>,
}

#[derive(Default)]
pub struct LoginFailed {
  reader: Option<OnTimerEventReader>,
}

impl LoginFailed {
  pub fn new() -> Self {
    Self { reader: None }
  }
}

impl<'a> System<'a> for LoginFailed {
  type SystemData = (Read<'a, OnTimerEvent>, LoginSystemData<'a>);

  fn setup(&mut self, res: &mut Resources) {
    self.reader = Some(res.fetch_mut::<OnTimerEvent>().register_reader());

    Self::SystemData::setup(res);
  }

  fn run(&mut self, (channel, data): Self::SystemData) {
    for evt in channel.read(self.reader.as_mut().unwrap()) {
      if evt.ty != *LOGIN_FAILED {
        continue;
      }

      let evt = match evt.data {
        Some(ref v) => match (*v).downcast_ref::<(ConnectionId, Login)>() {
          Some(v) => v.clone(),
          None => continue,
        },
        None => continue,
      };

      data.conns.send_to(
        evt.0,
        Error {
          error: ErrorType::Banned,
        },
      );
      data.conns.close(evt.0);
    }
  }
}

system_info! {
  impl SystemInfo for LoginFailed {
    type Dependencies = crate::handlers::OnCloseHandler;
  }
}

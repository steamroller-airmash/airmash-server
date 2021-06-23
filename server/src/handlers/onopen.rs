use crate::types::*;
use shrev::*;
use specs::*;

use crate::component::channel::{OnLogin, OnLoginReader};
use crate::task::new_connection;
use crate::types::event::ConnectionOpen;
use crate::utils::MaybeInit;

use crate::systems::PacketHandler;

#[derive(Default)]
pub struct OnOpenHandler {
  reader: MaybeInit<ReaderId<ConnectionOpen>>,
  login_reader: MaybeInit<OnLoginReader>,
}

#[derive(SystemData)]
pub struct OnOpenData<'a> {
  channel: Read<'a, EventChannel<ConnectionOpen>>,
  logins: Write<'a, OnLogin>,
  conns: Write<'a, Connections>,
  task: WriteExpect<'a, TaskSpawner>,
}

impl<'a> System<'a> for OnOpenHandler {
  type SystemData = OnOpenData<'a>;

  fn setup(&mut self, res: &mut Resources) {
    self.reader = MaybeInit::init(
      res
        .fetch_mut::<EventChannel<ConnectionOpen>>()
        .register_reader(),
    );
    self.login_reader = MaybeInit::init(res.fetch_mut::<OnLogin>().register_reader());

    Self::SystemData::setup(res);
  }

  fn run(&mut self, mut data: Self::SystemData) {
    let packets = data
      .logins
      .read(&mut self.login_reader)
      .cloned()
      .collect::<Vec<_>>();

    for evt in data.channel.read(&mut self.reader) {
      data
        .conns
        .add(evt.conn, evt.sink.clone(), evt.addr, evt.origin.clone());

      // Need to check for login packets that were received
      // before we had a chance to launch the task.
      let login = packets
        .iter()
        .filter(|(conn, _)| *conn == evt.conn)
        .map(|(_, packet)| packet.clone())
        .next();
      let tdata = data.task.task_data();
      let reader = data.logins.register_reader();
      data
        .task
        .launch(new_connection(tdata, evt.conn, reader, login));
    }
  }
}

system_info! {
  impl SystemInfo for OnOpenHandler {
    type Dependencies = PacketHandler;
  }
}

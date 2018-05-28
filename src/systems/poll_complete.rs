use specs::prelude::*;
use tokio::prelude::Sink;
use types::*;

use websocket::OwnedMessage;

use std::sync::mpsc::Receiver;

pub struct PollComplete {
    channel: Receiver<(ConnectionId, OwnedMessage)>
}

impl PollComplete {
    pub fn new(channel: Receiver<(ConnectionId, OwnedMessage)>) -> Self {
        Self { channel }
    }
}

impl<'a> System<'a> for PollComplete {
    type SystemData = Read<'a, Connections>;

    fn run(&mut self, conns: Self::SystemData) {
        while let Ok((id, msg)) = self.channel.try_recv() {
            match conns.0.get(&id) {
                Some(ref conn) => {
                    Connections::send_sink(&mut conn.sink.lock().unwrap(), msg);
                },
                // The connection probably closed,
                // do nothing
                None => ()
            }
        }

        for conn in conns.iter() {
            conn.sink.lock().unwrap().poll_complete().unwrap();
        }
    }
}

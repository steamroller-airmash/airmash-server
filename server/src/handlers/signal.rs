use specs::*;

use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use crate::consts::SHUTDOWN;
use crate::protocol::server::ServerMessage;
use crate::protocol::ServerMessageType;
use crate::types::systemdata::*;

use std::process;

#[derive(Default)]
pub struct SignalHandler {
  time: Option<Instant>,
}

impl<'a> System<'a> for SignalHandler {
  type SystemData = SendToAll<'a>;

  fn run(&mut self, data: Self::SystemData) {
    if SHUTDOWN.swap(false, Ordering::Relaxed) {
      if self.time.is_none() {
        self.time = Some(Instant::now());

        let msg = ServerMessage {
          duration: 15000,
          ty: ServerMessageType::Shutdown,
          text: "Server shutting down in 30 seconds!".to_string(),
        };

        data.send_to_all(msg);

        info!(
          target:"server",
          "Received interrupt, shutting down in 30s"
        );
      } else {
        info!("Received second interrupt, server shutting down NOW!");

        process::exit(0);
      }
    } else if self.time.is_some() {
      let t = self.time.unwrap();

      if Instant::now() - t > Duration::from_secs(30) {
        process::exit(0);
      }
    }
  }
}

use crate::dispatch::SystemInfo;

impl SystemInfo for SignalHandler {
  type Dependencies = ();

  fn new() -> Self {
    Self::default()
  }

  fn name() -> &'static str {
    concat!(module_path!(), "::", line!())
  }
}

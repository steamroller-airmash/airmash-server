use std::{
  collections::VecDeque,
  convert::TryInto,
  time::{Duration, Instant},
};

use airmash_protocol::client::Pong;

use crate::{component::*, event::PacketEvent, AirmashGame};

struct PingData {
  seqs: VecDeque<(u32, Instant)>,
}

impl PingData {
  pub fn new() -> Self {
    Self {
      seqs: Default::default(),
    }
  }

  fn push_seq(&mut self, seq: u32, time: Instant) {
    self.seqs.push_back((seq, time));

    if self.seqs.len() > 10 {
      self.seqs.pop_front();
    }
  }

  fn seq_time(&self, seq: u32) -> Option<Instant> {
    self
      .seqs
      .iter()
      .find(|&&(s, _)| s == seq)
      .map(|&(_, time)| time)
  }

  fn last_ping(&self) -> Option<Instant> {
    Some(self.seqs.back()?.1)
  }
}

pub fn update(game: &mut AirmashGame) {
  send_ping_packets(game);
}

fn send_ping_packets(game: &mut AirmashGame) {
  use crate::protocol::server::Ping;

  let this_frame = game.this_frame();
  let clock = crate::util::get_current_clock(game);
  let data = game
    .resources
    .entry::<PingData>()
    .or_insert_with(PingData::new);

  if data
    .last_ping()
    .map(|p| this_frame - p < Duration::from_secs(5))
    .unwrap_or(false)
  {
    return;
  }

  let seq = rand::random();
  data.push_seq(seq, this_frame);

  game.send_to_all(Ping { clock, num: seq });
}

#[handler]
fn handle_ping_response(event: &PacketEvent<Pong>, game: &mut AirmashGame) {
  use crate::protocol::server::PingResult;

  let this_frame = game.this_frame();
  let data = match game.resources.get::<PingData>() {
    Some(data) => data,
    None => return,
  };

  let ping: u16 = match data.seq_time(event.packet.num) {
    Some(time) => (this_frame - time)
      .as_millis()
      .try_into()
      .unwrap_or(u16::MAX),
    None => return,
  };

  if let Ok((player_ping, _)) = game
    .world
    .query_one_mut::<(&mut PlayerPing, &IsPlayer)>(event.entity)
  {
    player_ping.0 = ping;
  }

  game.send_to(
    event.entity,
    PingResult {
      ping,
      // TODO
      players_game: 0,
      players_total: 0,
    },
  )
}

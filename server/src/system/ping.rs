use std::collections::VecDeque;
use std::convert::TryInto;
use std::time::{Duration, Instant};

use airmash_protocol::client::Pong;

use crate::component::*;
use crate::event::PacketEvent;
use crate::resource::ServerStats;
use crate::AirmashGame;

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
  let clock = crate::util::get_time_clock(game, Instant::now());
  let data = game
    .resources
    .entry::<PingData>()
    .or_insert_with(PingData::new);

  if data
    .last_ping()
    .map(|p| this_frame.saturating_duration_since(p) < Duration::from_secs(5))
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

  let num_players = game.resources.read::<ServerStats>().num_players;
  let data = match game.resources.get::<PingData>() {
    Some(data) => data,
    None => return,
  };

  let ping = match data.seq_time(event.packet.num) {
    Some(time) => event.time.saturating_duration_since(time),
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
      ping: ping.as_millis().try_into().unwrap_or(u16::MAX),
      players_game: num_players,
      // TODO: Somehow get the total number of players from a server.
      players_total: num_players,
    },
  )
}

use crate::types::systemdata::*;
use crate::types::*;
use specs::*;

use crate::consts::timer::SCORE_BOARD;

use crate::component::event::TimerEvent;
use crate::component::flag::IsPlayer;
use crate::component::time::JoinTime;

use crate::protocol::server::{ScoreBoard, ScoreBoardData, ScoreBoardRanking};
use crate::utils::*;

use std::cmp::{Ordering, Reverse};

/// Collect an iterator of 2-tuples into a tuple of vectors
fn collect_tuples<T, U, I>(iter: I) -> (Vec<T>, Vec<U>)
where
  I: Iterator<Item = (T, U)>,
{
  iter.fold((vec![], vec![]), |mut acc, (a, b)| {
    acc.0.push(a);
    acc.1.push(b);
    acc
  })
}

/// When a SCORE_BOARD timer event shows up,
/// send a ScoreBoard packet to all players
#[derive(Default)]
pub struct ScoreBoardTimerHandler;

#[derive(SystemData)]
pub struct ScoreBoardSystemData<'a> {
  conns: SendToAll<'a>,

  entities: Entities<'a>,
  scores: ReadStorage<'a, Score>,
  levels: ReadStorage<'a, Level>,
  pos: ReadStorage<'a, Position>,
  is_player: ReadStorage<'a, IsPlayer>,
  is_alive: IsAlive<'a>,
  join_time: ReadStorage<'a, JoinTime>,
}

impl EventHandlerTypeProvider for ScoreBoardTimerHandler {
  type Event = TimerEvent;
}

impl<'a> EventHandler<'a> for ScoreBoardTimerHandler {
  type SystemData = ScoreBoardSystemData<'a>;

  fn on_event(&mut self, evt: &TimerEvent, data: &mut Self::SystemData) {
    if evt.ty != *SCORE_BOARD {
      return;
    }

    trace!("Sending SCORE_BOARD packet!");

    let mut packet_data = (
      &*data.entities,
      &data.scores,
      &data.levels,
      &data.join_time,
      &data.pos,
      data.is_player.mask(),
    )
      .join()
      .map(|(ent, score, level, join_time, pos, ..)| {
        let low_res_pos = if data.is_alive.get(ent) {
          Some(*pos)
        } else {
          None
        };

        (
          score.0,
          join_time.0,
          ScoreBoardData {
            id: ent.into(),
            score: *score,
            level: *level,
          },
          ScoreBoardRanking {
            id: ent.into(),
            pos: low_res_pos,
          },
        )
      })
      .collect::<Vec<_>>();

    // Sort all data first by score (descending)
    // then by join time (ascending)
    packet_data.sort_by(|a, b| {
      let ord = Reverse(a.0).cmp(&Reverse(b.0));

      match ord {
        Ordering::Equal => a.1.cmp(&b.1),
        _ => ord,
      }
    });

    let (mut sb_data, rankings) = collect_tuples(packet_data.into_iter().map(|x| (x.2, x.3)));

    // Only the top 10 players go for the
    // score board.
    sb_data.truncate(10);

    let score_board = ScoreBoard {
      data: sb_data,
      rankings,
    };

    data.conns.send_to_all(score_board);
  }
}

use crate::dispatch::SystemInfo;
use crate::systems::handlers::game::on_join::SendLogin;
use crate::systems::TimerHandler;

impl SystemInfo for ScoreBoardTimerHandler {
  type Dependencies = (TimerHandler, SendLogin);

  fn new() -> Self {
    Self::default()
  }

  fn name() -> &'static str {
    concat!(module_path!(), "::", line!())
  }
}

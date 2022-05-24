use crate::component::*;
use crate::resource::ThisFrame;
use crate::AirmashGame;
use std::cmp::Reverse;
use std::time::Duration;
use std::time::Instant;

def_wrappers! {
  type LastScoreBoardTime = Instant;
}

pub fn update(game: &mut AirmashGame) {
  send_packets(game);
}

fn send_packets(game: &mut AirmashGame) {
  use crate::protocol::server as s;

  let this_frame = game.resources.read::<ThisFrame>().0;
  let last_sb = game
    .resources
    .entry::<LastScoreBoardTime>()
    .or_insert(LastScoreBoardTime(this_frame));

  if this_frame < last_sb.0 + Duration::from_secs(2) {
    return;
  }

  last_sb.0 = this_frame;

  let mut data = Vec::new();
  let query = game
    .world
    .query_mut::<(&Position, &IsAlive, &Score, &Level, &JoinTime)>()
    .with::<IsPlayer>();
  for (player, (pos, alive, score, level, join_time)) in query {
    let low_res_pos = match alive.0 {
      true => Some(pos.0),
      false => None,
    };

    data.push((
      join_time.0,
      s::ScoreBoardData {
        id: player.id() as _,
        score: score.0,
        level: level.0,
      },
      s::ScoreBoardRanking {
        id: player.id() as _,
        pos: low_res_pos,
      },
    ));
  }

  data.sort_unstable_by_key(|x| (Reverse(x.1.score), x.0));

  let packet = s::ScoreBoard {
    rankings: data.iter().map(|x| x.2).collect(),
    data: data.into_iter().take(10).map(|x| x.1).collect(),
  };

  game.send_to_all(packet);
}

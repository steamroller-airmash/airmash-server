mod score_board;

pub use self::score_board::ScoreBoardTimer;

use crate::dispatch::Builder;

pub fn register<'a, 'b>(disp: Builder<'a, 'b>) -> Builder<'a, 'b> {
  disp.with::<ScoreBoardTimer>()
}

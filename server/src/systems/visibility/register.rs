use crate::dispatch::Builder;

use super::*;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
  builder.with::<GenPlayerGrid>().with::<TrackVisible>()
}

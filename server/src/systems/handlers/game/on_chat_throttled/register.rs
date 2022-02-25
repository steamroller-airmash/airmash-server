use super::*;
use crate::Builder;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
  builder.with::<SetUnthrottleTimer>()
}

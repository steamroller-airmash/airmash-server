use super::*;
use crate::dispatch::Builder;

pub fn register<'a, 'b>(disp: Builder<'a, 'b>) -> Builder<'a, 'b> {
	disp.with::<FrameCounter>()
}

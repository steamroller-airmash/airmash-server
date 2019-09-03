use crate::dispatch::Builder;
use crate::task;

use super::*;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	builder
		.with::<GenPlayerGrid>()
		.with_task(task::calculate_visibility)
	// .with::<TrackVisible>()
}

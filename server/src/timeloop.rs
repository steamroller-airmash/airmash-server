use futures::{Future, Stream};
use tokio::timer::Interval;

use std::time::{Duration, Instant};

pub fn timeloop<'a, 'b, F: FnMut(Instant) -> ()>(
	mut func: F,
	period: Duration,
) -> impl Future<Item = (), Error = ()> {
	Interval::new(Instant::now(), period)
		.take_while(move |_| Ok(true))
		.map_err(move |e| {
			error!(
				target: "server",
				"A timer error occurred: {}",
				e
			);
		}).for_each(move |instant| {
			trace!(
				target: "server",
				"Starting gameloop iteration at {:?}",
				instant
			);

			func(instant);

			trace!(
				target: "server",
				"Finished gameloop iteration at {:?}",
				Instant::now()
			);

			Ok(())
		})
}

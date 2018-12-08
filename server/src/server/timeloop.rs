use futures::{Future, Stream};
use tokio::timer::Interval;

use std::time::{Duration, Instant};

pub(crate) fn timeloop<F: FnMut(Instant) -> bool>(
	func: F,
	period: Duration,
) -> impl Future<Item = (), Error = ()> {
	Interval::new(Instant::now(), period)
		.map_err(move |e| {
			error!(
				target: "server",
				"A timer error occurred: {}",
				e
			);
		})
		.map(func)
		.take_while(|&x| Ok(x))
		.for_each(|_| Ok(()))
}

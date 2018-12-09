use std::error::Error;
use std::net::ToSocketAddrs;
use std::time::{Duration, Instant};

use specs::{Dispatcher, World};
use tokio::runtime::current_thread::Runtime;

use super::timeloop::timeloop;
use super::{spawn_acceptor, timers, AirmashServerConfig};
use component::time::{LastFrame, StartTime};

const FRAME_WARN_MILLIS: u64 = 60;

/// Airmash server instance.
///
/// Call [`run`][0] to run the server until
/// shutdown.
///
/// [0]: #fn.run
pub struct AirmashServer<T>
where
	T: ToSocketAddrs + Send + 'static,
{
	config: AirmashServerConfig<T>,
}

impl<T> AirmashServer<T>
where
	T: ToSocketAddrs + Send + 'static,
{
	fn game_loop(now: Instant, world: &mut World, dispatcher: &mut Dispatcher) -> bool {
		use component::time::{LastFrame, ThisFrame};

		let _now = Instant::now();

		if _now - now > Duration::from_millis(20) {
			// Drop a frame since we've drifted too far
			return true;
		}

		let now = _now;

		world.add_resource(ThisFrame(now));

		// A quick note on ordering here, world.maintain is called
		// before running thread local systems since ping packets
		// are setup during world.maintain. If this becomes a problem
		// then it'll be necessary to either redesign something or
		// to pull PollComplete and run it separately.
		dispatcher.dispatch_seq(&mut world.res);
		world.maintain();
		dispatcher.dispatch_thread_local(&mut world.res);

		world.add_resource(LastFrame(now));

		let duration = Instant::now() - now;
		if duration > Duration::from_millis(FRAME_WARN_MILLIS) {
			// Adjust this down once it becomes a more rare event
			warn!(
				"Frame took {} ms! (longer than {} ms)",
				1000 * duration.as_secs() + (duration.subsec_millis() as u64),
				FRAME_WARN_MILLIS
			);
		} else {
			trace!("Frame time: {} ms", duration.subsec_millis());
		}

		true
	}

	pub fn run(self) -> Result<(), Box<Error>> {
		let wshandler = spawn_acceptor(
			self.config.addr,
			self.config.event,
			self.config.max_connections,
		);

		let AirmashServerConfig {
			mut world,
			builder,
			timer,
			..
		} = self.config;

		world.add_resource(StartTime(Instant::now()));
		world.add_resource(LastFrame(Instant::now()));

		let mut dispatcher = builder.build();
		dispatcher.setup(&mut world.res);

		let mut runtime = Runtime::new()?;

		runtime.spawn(futures::lazy(move || {
			timers::start_timer_events(timer);

			Ok(())
		}));

		runtime.spawn(timeloop(
			move |now| Self::game_loop(now, &mut world, &mut dispatcher),
			Duration::from_nanos(16666667),
		));

		runtime.run()?;

		// FIXME: Somehow handle the case where the thread
		//        join fails and pass it on up to the caller
		wshandler.join().unwrap()?;

		Ok(())
	}

	pub fn new(config: AirmashServerConfig<T>) -> Self {
		Self { config }
	}
}

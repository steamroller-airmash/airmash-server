use parking_lot::RwLock;
use std::error::Error;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::time::{Duration, Instant};

use specs::{Dispatcher, World, WorldExt};

use super::{spawn_acceptor, AirmashServerConfig};
use crate::component::time::{LastFrame, StartTime};
use crate::task::{ExecutorHandle, TaskData};
use crate::types::TaskSpawner;

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
		use crate::component::time::ThisFrame;

		let _now = Instant::now();

		if _now - now > Duration::from_millis(20) {
			// Drop a frame since we've drifted too far
			return true;
		}

		let now = _now;

		world.insert(ThisFrame(now));

		dispatcher.dispatch_seq(world);
		dispatcher.dispatch_thread_local(world);
		world.maintain();

		world.insert(LastFrame(now));

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

	pub fn run(self) -> Result<(), Box<dyn Error>> {
		let wshandler = spawn_acceptor(
			self.config.addr,
			self.config.event,
			self.config.max_connections,
		);

		let AirmashServerConfig { world, builder, .. } = self.config;

		let world = Arc::new(RwLock::new(world));
		let mut executor = ExecutorHandle::new();

		world.write().insert(TaskSpawner::new(
			TaskData::new(Arc::clone(&world)),
			executor.clone(),
		));
		world.write().insert(StartTime(Instant::now()));
		world.write().insert(LastFrame(Instant::now()));

		let (mut dispatcher, tasks) = builder.build();
		dispatcher.setup(&mut world.write());

		// Setup initial tasks
		for task in tasks {
			let taskdata = TaskData::new(Arc::clone(&world));
			executor.spawn_fut_dyn(task(taskdata));
		}

		let frame_time = Duration::from_nanos(16666667);
		let mut next_frame = Instant::now();

		loop {
			let now = Instant::now();
			if now >= next_frame {
				// Poll all tasks
				executor.cycle();

				// Actually run the game loop
				if !Self::game_loop(next_frame, &mut world.write(), &mut dispatcher) {
					break;
				}
				next_frame += frame_time;

				let now = Instant::now();
				if next_frame <= now - Duration::from_millis(8) {
					let diff = now - next_frame;
					warn!(
						"Frame took too long, jumping forward {} ms",
						(now - next_frame).as_millis()
					);
					next_frame += diff;
				}
			} else {
				let wait_time = next_frame - now;

				if wait_time < Duration::from_millis(1) {
					// Do a spin loop for low sleep durations
				} else {
					std::thread::sleep(wait_time - Duration::from_millis(1));
				}
			}
		}

		info!("Shutting down airmash server");

		// FIXME: Somehow handle the case where the thread
		//        join fails and pass it on up to the caller
		wshandler.join().unwrap()?;

		Ok(())
	}

	pub fn new(config: AirmashServerConfig<T>) -> Self {
		Self { config }
	}
}

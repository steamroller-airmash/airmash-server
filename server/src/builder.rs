use std::fmt::Debug;
use std::net::ToSocketAddrs;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use specs::Builder as SpecsBuilder;
use specs::*;

use futures;

use dispatch::Builder;
use server;
use systems;
use timeloop::timeloop;
use timers;

use types::connection::Message;
use types::event::ConnectionEvent;
use types::{Connections, FutureDispatcher, GameMode};

use component::event::TimerEvent;
use component::time::{LastFrame, StartTime, ThisFrame};

use tokio::runtime::current_thread::Runtime;

struct Channel<T> {
	send: Option<Sender<T>>,
	recv: Option<Receiver<T>>,
}

impl<T> Channel<T> {
	pub fn new() -> Self {
		let (send, recv) = channel();

		Self {
			send: Some(send),
			recv: Some(recv),
		}
	}
}

pub struct AirmashServer<'a, 'b, T>
where
	T: ToSocketAddrs + Debug + Send + 'static,
{
	pub builder: Builder<'a, 'b>,
	addr: T,

	event: Channel<ConnectionEvent>,
	timer: Channel<TimerEvent>,
	msg: Channel<Message>,

	pub world: World,
}

impl<T> AirmashServer<'static, 'static, T>
where
	T: ToSocketAddrs + Debug + Send + 'static,
{
	pub fn new(addr: T) -> Self {
		Self {
			builder: Builder::new(),
			addr: addr,

			event: Channel::new(),
			timer: Channel::new(),
			msg: Channel::new(),

			world: World::new(),
		}
	}

	pub fn with_base_systems(self) -> Self {
		let Self {
			builder,
			addr,
			event,
			timer,
			msg,
			world,
		} = self;

		// Register
		let builder = builder
			.with_args::<systems::PacketHandler, _>(event.recv.unwrap())
			.with_args::<systems::TimerHandler, _>(timer.recv.unwrap())
			.with_thread_local_args::<systems::PollComplete, _>(msg.recv.unwrap());

		Self {
			builder,
			addr,
			world,

			event: Channel {
				send: event.send,
				recv: None,
			},
			timer: Channel {
				send: timer.send,
				recv: None,
			},
			msg: Channel {
				send: msg.send,
				recv: None,
			},
		}
	}

	pub fn with_engine_systems(self) -> Self {
		let Self {
			builder,
			addr,
			event,
			timer,
			msg,
			world,
		} = self.with_base_systems();

		let builder = systems::register(builder);

		Self {
			builder,
			addr,
			event,
			timer,
			msg,
			world,
		}
	}

	pub fn with_engine_resources(self) -> Self {
		let Self {
			builder,
			addr,
			event,
			timer,
			msg,
			mut world,
			..
		} = self;

		world.add_resource(Connections::new(msg.send.unwrap()));
		world.add_resource(FutureDispatcher::new(timer.send.as_ref().unwrap().clone()));

		Self {
			builder,
			addr,
			event,
			timer,
			world,

			msg: Channel {
				send: None,
				recv: msg.recv,
			},
		}
	}

	/// Add some dummmy entities so that there are no players with id 0, 1, or 2
	/// this makes FFA team logic easier. StarMash also appears to
	/// make all players mimic the player with id 0
	pub fn with_filler_entities(mut self) -> Self {
		while self.world.create_entity().build().id() < 3 {}

		self
	}

	pub fn with_engine(self) -> Self {
		self.with_engine_systems()
			.with_engine_resources()
			.with_filler_entities()
	}

	pub fn with_gamemode<G>(mut self, mode: G) -> Self
	where
		G: GameMode + 'static,
	{
		use types::gamemode::*;

		let val = GameModeInternal(Box::new(GameModeWrapperImpl { val: mode }));

		self.world.add_resource(val);
		self
	}

	pub fn with_alpha_warning(self) -> Self {
		use systems::notify::*;

		Self {
			builder: self.builder.with::<NotifyAlpha>(),
			..self
		}
	}

	pub fn run(self) {
		let Self {
			builder,
			addr,
			event,
			timer,
			mut world,
			..
		} = self;

		info!("Starting server runtime!");

		// The acceptor needs to run on its own thread
		// to avoid delaying packets
		let server_thread = thread::spawn(move || {
			server::run_acceptor(addr, event.send.unwrap());
		});

		world.add_resource(StartTime(Instant::now()));

		let mut dispatcher = builder.build();
		dispatcher.setup(&mut world.res);

		world.add_resource(LastFrame(Instant::now()));

		let mut runtime = Runtime::new().unwrap();

		runtime.spawn(futures::lazy(move || {
			timers::start_timer_events(timer.send.unwrap());

			Ok(())
		}));

		runtime.spawn(timeloop(
			move |now| {
				if Instant::now() - now > Duration::from_millis(30) {
					//warn!("Time has drifted more than 30 ms, skipping frame!");
					return;
				}

				world.add_resource(ThisFrame(now));
				dispatcher.dispatch_seq(&mut world.res);
				dispatcher.dispatch_thread_local(&mut world.res);
				world.maintain();
				world.add_resource(LastFrame(now));

				let duration = Instant::now() - now;
				if duration > Duration::from_millis(30) {
					// Adjust this down once it becomes a more rare event
					warn!(
						"Frame took {} ms! (longer than 30 ms)",
						1000 * duration.as_secs() + (duration.subsec_millis() as u64)
					);
				} else {
					trace!("Frame time: {} ms", duration.subsec_millis());
				}
			},
			Duration::from_nanos(16666667),
		));

		runtime.run().unwrap();

		// Shut down
		info!("Exited game loop, shutting down");
		server_thread.join().unwrap();
	}
}

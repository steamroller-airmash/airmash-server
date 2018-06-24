
use std::env;
use std::sync::atomic::Ordering;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use specs::{Dispatcher, World};
use tokio::runtime::current_thread::Runtime;

use component::event::TimerEvent;
use component::time::{LastFrame, StartTime, ThisFrame};
use dispatch::Builder;
use timeloop::timeloop;

use types::event::ConnectionEvent;

use types;
use websocket;
use systems;
use consts;
use ctrlc;
use simple_logger;
use tokio;
use server;
use timers;
use futures;
use log;
use metrics;

fn build_dispatcher<'a, 'b>(
	world: &mut World,
	event_recv: Receiver<ConnectionEvent>,
	timer_recv: Receiver<TimerEvent>,
	msg_recv: Receiver<(types::ConnectionId, websocket::OwnedMessage)>,
) -> Dispatcher<'a, 'b> {
	let disp = Builder::new()
		// Add systems here
		.with_args::<systems::PacketHandler, _>(event_recv)
		.with_args::<systems::TimerHandler, _>(timer_recv);

	let disp = systems::register(disp);
	let disp = systems::ctf::register(world, disp);

	disp
		// This needs to run after systems which send messages
		.with_thread_local_args::<systems::PollComplete, _>(msg_recv)

		// Build
		.build()
}

fn setup_panic_handler() {
	use std::panic;
	use std::process;

	let orig_handler = panic::take_hook();
	panic::set_hook(Box::new(move |panic_info| {
		if consts::SHUTDOWN.load(Ordering::Relaxed) {
			// This is a normal shutdown
			// no need to print to the log
			process::exit(0);
		}
		error!("A fatal error occurred within a server thread. Aborting!");
		error!("Error Info: {}", panic_info);

		orig_handler(panic_info);
		process::exit(1);
	}));
}

fn setup_interrupt_handler() {
	ctrlc::set_handler(move || {
		consts::SHUTDOWN.store(true, Ordering::Relaxed);
	}).expect("Error setting iterrupt handler");
}

pub fn run_server() {
	simple_logger::init_with_level(log::Level::Info).unwrap();
	env::set_var("RUST_BACKTRACE", "1");
	// Quick hack to change threadpool size
	env::set_var("RAYON_NUM_THREADS", "1");

	setup_panic_handler();
	setup_interrupt_handler();

	let addr = "0.0.0.0:3501";

	let mut world = World::new();

	let (event_send, event_recv) = channel::<ConnectionEvent>();
	let (timer_send, timer_recv) = channel::<TimerEvent>();
	let (msg_send, msg_recv) = channel::<(types::ConnectionId, websocket::OwnedMessage)>();

	// Add resources
	info!("Setting up resources");

	let metric_handler = metrics::handler();

	world.add_resource(metric_handler.clone());
	world.add_resource(types::Connections::new(msg_send));
	world.add_resource(types::FutureDispatcher::new(timer_send.clone()));

	// Add systems
	info!("Setting up systems");

	let mut dispatcher = build_dispatcher(&mut world, event_recv, timer_recv, msg_recv);

	// Start websocket server
	info!("Starting websocket server!");
	let server_thread = thread::spawn(move || {
		server::run_acceptor(addr, event_send);
	});

	// Start gameloop
	info!("Starting gameloop!");

	// Need to run the event loop on the current
	// thread since Dispatcher doesn't implement Send
	let mut runtime = Runtime::new().unwrap();

	// Start timer loops
	let timers = thread::spawn(move || {
		tokio::run(futures::lazy(move || {
			timers::start_timer_events(timer_send);
			Ok(())
		}));
	});

	world.add_resource(StartTime(Instant::now()));
	dispatcher.setup(&mut world.res);
	world.add_resource(LastFrame(Instant::now()));

	// Add some dummmy entities so that there are no players with id 0, 1, or 2
	// this makes FFA team logic easier. StarMash also appears to
	// make all players mimic the player with id 0
	for _ in 0..3 {
		world.create_entity().build();
	}

	// Run the gameloop at 60 Hz
	runtime.spawn(timeloop(
		move |now| {
			if Instant::now() - now > Duration::from_millis(30) {
				warn!("Time has drifted more than 30 ms, skipping frame!");
				return;
			}

			world.add_resource(ThisFrame(now));
			dispatcher.dispatch_seq(&mut world.res);
			dispatcher.dispatch_thread_local(&mut world.res);
			world.maintain();
			world.add_resource(LastFrame(now));

			let duration = Instant::now() - now;
			if duration > Duration::from_millis(17) {
				warn!(
					"Frame took {} ms! (longer than 16.67 ms)",
					1000 * duration.as_secs() + (duration.subsec_millis() as u64)
				);
			} else {
				trace!("Frame time: {} ms", duration.subsec_millis());
			}

			// Don't crash server if metric recording fails
			metric_handler
				.time_duration("frame-time", duration)
				.unwrap();
		},
		Duration::from_nanos(16666667),
	));

	runtime.run().unwrap();

	// Shut down
	info!(target: "server", "Exited gameloop, shutting down");
	server_thread.join().unwrap();
	timers.join().unwrap();

	info!(target: "server", "Shutdown completed successfully");
}

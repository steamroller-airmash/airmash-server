
#![allow(dead_code, unused_imports)]
#![feature(optin_builtin_traits)]

// Crates with macros
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate dimensioned;
#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate shred_derive;

// Regular Dependencies
extern crate simple_logger;
extern crate rand;
extern crate specs;
extern crate shrev;
extern crate shred;
extern crate fnv;
extern crate uuid;
extern crate tokio;
extern crate tokio_core;
extern crate websocket;
extern crate airmash_protocol;

use websocket::futures as futures;

// Modules
mod types;
mod server;
mod systems;
mod handlers;
mod gameloop;

use std::env;
use std::thread;
use std::sync::Mutex;
use std::time::Duration;
use std::sync::mpsc::{channel, Receiver};

use specs::{World, DispatcherBuilder, Dispatcher};
use tokio::reactor::Reactor;
use tokio::runtime::current_thread::Runtime;

use types::{ThisFrame, LastFrame};

fn build_dispatcher<'a, 'b>(
    event_recv: Receiver<types::ConnectionEvent>
) -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        // Add systems here
        .with(systems::PacketHandler::new(event_recv), "packet", &[])
        
        // Add handlers here
        .with(handlers::OnOpenHandler::new(),  "onopen",  &["packet"])
        .with(handlers::OnCloseHandler::new(), "onclose", &["packet", "onopen"])
        .with(handlers::LoginHandler::new(),   "onlogin", &["packet", "onclose"])

        // This needs to run after all messages are sent
        .with(systems::PollComplete::new(),    "poll-complete", &[
            "onopen", "onclose", "onlogin", "packet"
        ])

        // Build
        .build()
}

fn setup_panic_handler() {
    use std::panic;
    use std::process;

	let orig_handler = panic::take_hook();
	panic::set_hook(Box::new(move |panic_info| {
		error!(
			target: "server",
			"A fatal error occurred within a server thread. Aborting!",
		);
		error!(
			target: "server",
			"Error Info: {}",
			panic_info
		);

		orig_handler(panic_info);
		process::exit(1);
	}));
}

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    env::set_var("RUST_BACKTRACE", "1");

    setup_panic_handler();

    let addr = "127.0.0.1:3501";

    let mut world = World::new();

    let (event_send, event_recv) = channel::<types::ConnectionEvent>();

    // Add resources
    info!(target: "server", "Setting up resources");

    world.add_resource(types::Connections::new());

    // Add systems
    info!(target: "server", "Setting up systems");

    let mut dispatcher = build_dispatcher(event_recv);

    // Start websocket server
    info!(target: "server", "Starting websocket server!");
    let server_thread = thread::spawn(move || {
        server::run_acceptor(addr, event_send);
    });

    // Start gameloop
    info!(target: "server", "Starting gameloop!");
    
    // Need to run the event loop on the current
    // thread since Dispatcher doesn't implement Send
    let mut runtime = Runtime::new().unwrap();

    let background = Reactor::new()
        .unwrap()
        .background()
        .unwrap();

    world.add_resource(background.handle().clone());

    dispatcher.setup(&mut world.res);

    // Run the gameloop at 60 Hz
    runtime.spawn(gameloop::gameloop(move |now| {
        world.add_resource(ThisFrame(now));
        dispatcher.dispatch(&mut world.res);
        world.maintain();
        world.add_resource(LastFrame(now));
    }, Duration::from_nanos(16666667)));

    runtime.run().unwrap();

    // Shut down
    info!(target: "server", "Exited gameloop, shutting down");
    server_thread.join().unwrap();

    info!(target: "server", "Shutdown completed successfully");
}

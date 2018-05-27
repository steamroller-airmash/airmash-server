
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
extern crate tokio_core;
extern crate websocket;
extern crate airmash_protocol;

use websocket::futures as futures;

// Modules
mod types;
mod server;
mod systems;
mod handlers;

use std::env;
use std::thread;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver};

use specs::{World, DispatcherBuilder, Dispatcher};

fn build_dispatcher<'a, 'b>(
    event_recv: Receiver<types::ConnectionEvent>
) -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        // Add systems here
        .with(systems::PacketHandler::new(event_recv), "packet-handler", &[])
        
        // Add handlers here
        .with(handlers::LoginHandler::new(), "login-handler", &["packet-handler"])

        // Build
        .build()
}

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    env::set_var("RUST_BACKTRACE", "1");

    let addr = "ws://127.0.0.1:3501";

    let mut world = World::new();

    let (event_send, event_recv) = channel::<types::ConnectionEvent>();

    // Add resources
    info!(target: "server", "Setting up resources!");

    world.add_resource(types::Connections::new());

    // Add systems
    info!(target: "server", "Setting up systems!");

    let mut dispatcher = build_dispatcher(event_recv);

    // Start websocket server
    info!(target: "server", "Starting websocket server!");
    let server_thread = thread::spawn(move || {
        server::run_acceptor(addr, event_send);
    });

    // Start gameloop
    info!(target: "server", "Starting gameloop!");


    // Shut down
    info!(target: "server", "Exited gameloop, shutting down");
    server_thread.join().unwrap();

    info!(target: "server", "Shutdown completed successfully");
}

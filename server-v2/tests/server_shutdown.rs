use server_v2::ecs::*;
use server_v2::resource::builtin::ShutdownFlag;
use server_v2::server::*;

use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, Instant};

use tokio::task::LocalSet;

#[derive(Default)]
struct ShutdownSystem;

impl<'a> System<'a> for ShutdownSystem {
    type SystemData = WriteExpect<'a, ShutdownFlag>;

    fn run(&mut self, mut data: Self::SystemData) {
        data.shutdown();
    }
}

impl SystemBuilder for ShutdownSystem {
    type System = Self;
    type Dependencies = ();

    fn build(self) -> Self {
        self
    }
}

#[test]
fn server_shutdown() {
    let (tx, rx) = channel();

    let handle = thread::spawn(move || {
        let mut world = World::new();
        let mut builder = Dispatcher::builder(&mut world);

        builder.with::<ShutdownSystem>();

        let dispatch = builder.build().expect("Failed to build dispatcher");

        let config = AirmashServerConfig::default();
        let server = AirmashServer::new(dispatch, world, LocalSet::new(), config);

        server.run().expect("Failed to run server");

        tx.send(()).unwrap();
    });

    let start = Instant::now();

    loop {
        if start + Duration::from_secs(5) < Instant::now() {
            panic!("Server shutdown didn't work!");
        }

        thread::sleep(Duration::from_millis(100));

        match rx.try_recv() {
            Ok(_) => break,
            Err(_) => (),
        }
    }

    handle.join().unwrap();
}

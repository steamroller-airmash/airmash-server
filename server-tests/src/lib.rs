//!

#![feature(termination_trait_lib)]

pub extern crate client;

pub use airmash_protocol as protocol;

use std::any::Any;
use std::future::Future;
use std::panic::{self, AssertUnwindSafe};
use std::pin::Pin;
use std::process::Termination;
use std::sync::mpsc::channel;
use std::task::{Context, Poll};
use std::thread;

use tokio::net::TcpStream;
use tokio::time::{delay_for, Duration};
use url::Url;

use client::{Client, ClientResult};
use server_v2::{
    ecs::{Builder, Entity, World},
    protocol::GameType,
    server::{AirmashServerBuilder, AirmashServerConfig},
    util::GameMode,
    Position, Team,
};

pub struct TestRunner {
    url: Url,
}

impl TestRunner {
    fn new(url: Url) -> Self {
        Self { url }
    }

    pub async fn new_client(&self) -> ClientResult<Client<TcpStream>> {
        Client::connect(self.url.clone()).await
    }
}

pub async fn run_test<T, F, R>(test: T, name: &str) -> bool
where
    T: FnOnce(TestRunner) -> F,
    F: Future<Output = R>,
    R: Termination,
{
    let (tx, rx) = channel();

    eprintln!("Starting test {}", name);

    let handle = thread::spawn(move || {
        let mut world = World::new();
        let mut builder = Builder::new(&mut world);

        builder.with_registrar(server_v2::system::register);
        let dispatch = builder.build().expect("Failed to schedule systems");

        let config = AirmashServerConfig::default();

        tx.send(()).unwrap();

        AirmashServerBuilder::new(world, config, EmptyGameMode, dispatch)
            .build()
            .run()
            .unwrap();
    });

    if let Err(_) = rx.recv() {
        let _ = handle.join();
        eprintln!("... Test failed");
        return false;
    }

    delay_for(Duration::from_millis(10)).await;

    let url: Url = "ws://localhost:3501".parse().unwrap();
    let res = CatchPanic(test(TestRunner::new(url.clone()))).await;

    let rep = match res {
        Ok(res) => res.report(),
        Err(_) => 1,
    };

    if rep != 0 {
        eprintln!("... Test Failed");
    }

    kill_server(TestRunner::new(url)).await.unwrap();

    let _ = handle.join();

    rep == 0
}

async fn kill_server(runner: TestRunner) -> ClientResult<()> {
    let mut client = runner.new_client().await?;

    client.login("QuitBot").await?;
    client.send_command("shutdown", "").await?;
    client.quit().await?;

    Ok(())
}

struct EmptyGameMode;

impl GameMode for EmptyGameMode {
    fn assign_team(&mut self, player: Entity) -> Team {
        Team(player.id() as u16 + 3)
    }
    fn spawn_pos(&mut self, _: Entity, _: Team) -> Position {
        Position::default()
    }
    fn gametype(&self) -> GameType {
        GameType::FFA
    }
    fn room(&self) -> String {
        "matrix".to_owned()
    }
}

struct CatchPanic<F>(F);

impl<F: Future> Future for CatchPanic<F> {
    type Output = Result<F::Output, Box<dyn Any>>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let inner = unsafe { self.map_unchecked_mut(|me| &mut me.0) };
        let res = panic::catch_unwind(AssertUnwindSafe(|| inner.poll(ctx)));

        match res {
            Ok(x) => x.map(Ok),
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

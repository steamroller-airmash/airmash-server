//!

#![feature(termination_trait_lib)]

pub extern crate client;

pub use airmash_protocol as protocol;

use std::any::Any;
use std::future::Future;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::panic::{self, AssertUnwindSafe};
use std::pin::Pin;
use std::process::Termination;
use std::sync::{
    atomic::{AtomicU16, Ordering},
    mpsc::channel,
    Mutex,
};
use std::task::{Context, Poll};
use std::thread;

use once_cell::{sync::Lazy, sync_lazy};
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

pub async fn run_test<T, F, R>(test: T) -> R
where
    T: FnOnce(TestRunner) -> F,
    F: Future<Output = R>,
    R: Termination,
{
    let socket = SOCKETS.get_socket();
    let res = CatchPanic(run_test_inner(test, socket)).await;

    SOCKETS.return_socket(socket);

    match res {
        Ok(x) => x,
        Err(e) => std::panic::resume_unwind(e),
    }
}

async fn run_test_inner<T, F, R>(test: T, socket: SocketAddr) -> R
where
    T: FnOnce(TestRunner) -> F,
    F: Future<Output = R>,
    R: Termination,
{
    eprintln!("Creating server on {}", socket);

    let (tx, rx) = channel();

    let handle = thread::spawn(move || {
        let mut world = World::new();
        let mut builder = Builder::new(&mut world);

        builder.with_registrar(server_v2::system::register);
        let dispatch = builder.build().expect("Failed to schedule systems");

        let mut config = AirmashServerConfig::default();
        config.socket = socket;

        tx.send(()).unwrap();

        AirmashServerBuilder::new(world, config, EmptyGameMode, dispatch)
            .build()
            .run()
            .unwrap();
    });

    if let Err(_) = rx.recv() {
        let _ = handle.join();
        panic!("Server shut down abnormally!");
    }

    delay_for(Duration::from_millis(10)).await;

    let url: Url = format!("ws://{}", socket).parse().unwrap();
    let res = CatchPanic(test(TestRunner::new(url.clone()))).await;

    kill_server(TestRunner::new(url)).await.unwrap();

    let _ = handle.join();

    match res {
        Ok(x) => x,
        Err(e) => std::panic::resume_unwind(e),
    }
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
    type Output = Result<F::Output, Box<dyn Any + Send + 'static>>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let inner = unsafe { self.map_unchecked_mut(|me| &mut me.0) };
        let res = panic::catch_unwind(AssertUnwindSafe(|| inner.poll(ctx)));

        match res {
            Ok(x) => x.map(Ok),
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

static SOCKETS: Lazy<SocketManager> = sync_lazy! { SocketManager::new() };

struct SocketManager {
    available: Mutex<Vec<SocketAddr>>,
    next: AtomicU16,
}

impl SocketManager {
    pub fn new() -> Self {
        Self {
            available: Mutex::new(Vec::new()),
            next: AtomicU16::new(3502),
        }
    }

    pub fn get_socket(&self) -> SocketAddr {
        let mut available = self.available.lock().unwrap();

        match available.pop() {
            Some(x) => x,
            None => {
                let port = self.next.fetch_add(1, Ordering::Relaxed);
                SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port)
            }
        }
    }

    pub fn return_socket(&self, socket: SocketAddr) {
        let mut available = self.available.lock().unwrap();
        available.push(socket);
    }
}

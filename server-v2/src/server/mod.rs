//! Airmash server setup and config.

mod config;
mod websocket;

pub use self::config::AirmashServerConfig;

use std::cell::RefCell;
use std::error::Error;
use std::panic;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use tokio::runtime::Builder;
use tokio::task::LocalSet;

use self::websocket::websocket_listener;
use crate::ecs::{Dispatcher, World};
use crate::resource::builtin::{CurrentFrame, LastFrame, PlayerCount, ShutdownFlag, StartTime};
use crate::resource::socket::{OnClose, OnConnect, OnMessage};
use crate::util::{GameMode, GameModeInternal, GameModeWrapperImpl};

static PANIC_FLAG: AtomicBool = AtomicBool::new(false);

pub struct AirmashServerBuilder {
    config: AirmashServerConfig,
    world: Rc<RefCell<World>>,
    localset: LocalSet,
    dispatch: Dispatcher,
}

impl AirmashServerBuilder {
    pub fn without_gamemode(
        world: World,
        config: AirmashServerConfig,
        dispatch: Dispatcher,
    ) -> Self {
        Self {
            world: Rc::new(RefCell::new(world)),
            config,
            localset: LocalSet::new(),
            dispatch,
        }
    }

    pub fn new<G: GameMode>(
        mut world: World,
        config: AirmashServerConfig,
        gamemode: G,
        dispatch: Dispatcher,
    ) -> Self {
        world.register_resource(GameModeInternal(Box::new(GameModeWrapperImpl {
            val: gamemode,
        })));

        Self::without_gamemode(world, config, dispatch)
    }

    pub fn build(self) -> AirmashServer {
        AirmashServer {
            dispatch: self.dispatch,
            world: self.world,
            config: self.config,
            localset: self.localset,
        }
    }
}

pub struct AirmashServer {
    dispatch: Dispatcher,
    world: Rc<RefCell<World>>,
    config: AirmashServerConfig,
    localset: LocalSet,
}

impl AirmashServer {
    fn register_builtins(&mut self) {
        let mut world = self.world.borrow_mut();

        world.register_resource(ShutdownFlag::new());

        // Some async systems might see these before they are setup
        // in run_server. Set them here to valid values.
        world.register_resource(StartTime(Instant::now()));
        world.register_resource(LastFrame(Instant::now()));
        world.register_resource(CurrentFrame(Instant::now()));

        // Even if no systems make use of this we'll still need it
        // to be registered so that the endpoint service can use it.
        world.register_resource(PlayerCount(0));

        // Channels needed to handle connection-related things.
        world.register_resource_lazy(OnConnect::default);
        world.register_resource_lazy(OnMessage::default);
        world.register_resource_lazy(OnClose::default);

        // Useful for tasks and such.
        world.register_resource(Rc::downgrade(&self.world));
    }

    // TODO: This hack breaks the test framework.
    #[allow(dead_code)]
    fn setup_panic_hook(&mut self) {
        let hook = panic::take_hook();
        panic::set_hook(Box::new(move |panicinfo| {
            PANIC_FLAG.store(true, Ordering::Relaxed);
            hook(panicinfo)
        }));
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        self.register_builtins();
        // self.setup_panic_hook();

        let Self {
            world,
            dispatch,
            config,
            localset,
        } = self;

        let mut runtime = Builder::new()
            .basic_scheduler()
            .enable_time()
            .enable_io()
            .build()?;

        info!("Starting server on {}", config.socket);

        localset.spawn_local(websocket_listener(Rc::clone(&world), config.socket));

        localset.block_on(&mut runtime, Self::run_server(world, dispatch, config))
    }

    async fn run_server(
        world: Rc<RefCell<World>>,
        mut dispatch: Dispatcher,
        config: AirmashServerConfig,
    ) -> Result<(), Box<dyn Error>> {
        let mut current_frame = Instant::now() - Duration::from_millis(1);
        let mut interval =
            tokio::time::interval_at(tokio::time::Instant::now(), config.frame_duration());

        loop {
            let now = interval.tick().await;
            let mut world = world.borrow_mut();

            // Setup frame times
            world.register_resource(LastFrame(current_frame));
            world.register_resource(CurrentFrame(now.into_std()));
            current_frame = now.into_std();

            dispatch.dispatch_all(&mut world);
            world.maintain();

            let shutdown = world.fetch_resource::<ShutdownFlag>();

            if shutdown.value() {
                break;
            }

            if PANIC_FLAG.swap(false, Ordering::Relaxed) {
                panic!("Another tokio task/thread panicked. Exiting tokio runtime.");
            }
        }

        Ok(())
    }
}

use crate::ecs::{prelude::*, World, WorldEntityBuilder, Component};
use crate::system::builtin::AwakenQueue;

use std::cell::RefCell;
use std::future::Future;
use std::rc::{Rc, Weak};

use tokio::sync::oneshot::channel;
use tokio::task::spawn_local;

#[derive(SystemData)]
pub struct TaskSpawner<'a> {
    world: Read<'a, Weak<RefCell<World>>>,
}

impl TaskSpawner<'_> {
    pub fn spawn_raw<Fun, Fut>(&self, fun: Fun)
    where
        Fun: FnOnce(Rc<RefCell<World>>) -> Fut,
        Fut: Future<Output = ()> + 'static,
    {
        // This should always succeed when used from within
        // a system.
        let world = self
            .world
            .upgrade()
            .expect("TaskSpawner used outside of a system");

        spawn_local(fun(world));
    }

    pub fn spawn<Fun, Fut>(&self, fun: Fun)
    where
        Fun: FnOnce(TaskData) -> Fut,
        Fut: Future<Output = ()> + 'static,
    {
        self.spawn_raw(move |world| fun(TaskData::new(world)))
    }
}

pub struct TaskData {
    world: Rc<RefCell<World>>,
}

impl TaskData {
    fn new(world: Rc<RefCell<World>>) -> Self {
        Self { world }
    }

    pub async fn yield_frame(&mut self) {
        let rx = self.write_resource::<AwakenQueue, _, _>(|mut queue| {
            let (tx, rx) = channel();
            queue.0.push(tx);
            rx
        });

        let _ = rx.await;
    }

    pub fn world<F, R>(&mut self, fun: F) -> R
    where
        F: FnOnce(&mut World) -> R,
    {
        let mut world = self.world.borrow_mut();
        fun(&mut world)
    }

    pub fn read_resource<T, F, R>(&mut self, fun: F) -> R
    where
        F: FnOnce(ReadExpect<T>) -> R,
        T: 'static,
    {
        self.world(move |world| fun(world.system_data()))
    }

    pub fn write_resource<T, F, R>(&mut self, fun: F) -> R
    where
        F: FnOnce(WriteExpect<T>) -> R,
        T: 'static,
    {
        self.world(move |world| fun(world.system_data()))
    }

    pub fn read_storage<T, F, R>(&mut self, fun: F) -> R
    where
        F: FnOnce(ReadStorage<T>) -> R,
        T: Component + 'static,
    {
        self.world(move |world| fun(world.system_data()))
    }

    pub fn write_storage<T, F, R>(&mut self, fun: F) -> R
    where
        F: FnOnce(WriteStorage<T>) -> R,
        T: Component + 'static
    {
        self.world(move |world| fun(world.system_data()))
    }

    pub fn create_entity<F, R>(&mut self, fun: F) -> R
    where
        F: FnOnce(WorldEntityBuilder) -> R,
    {
        self.world(move |world| {
            let builder = world.create_entity();

            fun(builder)
        })
    }
}

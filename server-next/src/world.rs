use crate::event::ServerStartup;
use crate::network::ConnectionMgr;
use crate::{Event, EventDispatcher, EventHandler};
use airmash_protocol::GameType;
use anymap::AnyMap;
use hecs::Entity;
use std::cell::{Ref, RefCell, RefMut};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct AirmashGame {
  pub world: hecs::World,
  pub resources: Resources,
  pub(crate) dispatcher: EventDispatcher,

  shutdown: Arc<AtomicBool>,
}

impl AirmashGame {
  pub fn run_once(&mut self, now: Instant) {
    use crate::resource::*;

    {
      let mut last_frame = self.resources.write::<LastFrame>();
      let mut this_frame = self.resources.write::<ThisFrame>();

      last_frame.0 = this_frame.0;
      this_frame.0 = now;
    }

    crate::system::update(self);
  }

  pub fn run_until_shutdown(&mut self) {
    self.dispatch(ServerStartup);

    let timestep = Duration::from_secs_f32(1.0 / 60.0);
    let mut current = Instant::now();

    while !self.shutdown.load(Ordering::Relaxed) {
      current += timestep;
      let now = Instant::now();

      // If we're falling behind then skip a frame
      if current < now {
        continue;
      }

      if current - now > Duration::from_millis(1) {
        std::thread::sleep(current - now);
      }

      self.run_once(current);
    }
  }
}

impl AirmashGame {
  pub fn uninit() -> Self {
    AirmashGame {
      world: hecs::World::new(),
      resources: Resources::new(),
      dispatcher: EventDispatcher::new(),
      shutdown: Arc::new(AtomicBool::new(false)),
    }
  }

  /// An airmash server with the full networking backend enabled.
  pub fn with_network(addr: SocketAddr) -> Self {
    let mut me = Self::with_test_defaults();
    me.resources
      .insert(ConnectionMgr::with_server(addr, me.shutdown.clone()));

    me
  }

  /// An airmash server with all the functionality needed for testing
  pub fn with_test_defaults() -> Self {
    let mut me = Self::uninit();
    me.init_defaults();
    me
  }

  fn init_defaults(&mut self) {
    use crate::resource::collision::*;
    use crate::resource::*;

    let now = Instant::now();

    self.resources.insert(StartTime(now));
    self.resources.insert(LastFrame(now));
    self.resources.insert(ThisFrame(now));
    self.resources.insert(Config::default());
    self.resources.insert(Terrain::default());
    self.resources.insert(PlayerPosDb(SpatialTree::new()));
    self.resources.insert(PlayerCollideDb(SpatialTree::new()));
    self.resources.insert(MissileCollideDb(SpatialTree::new()));
    self.resources.insert(TaskScheduler::new());
    self.resources.insert(GameConfig::default());

    self.resources.insert(GameRoom("default-uninit".to_owned()));
    self.resources.insert(GameType::FFA);

    // Having entities with id 0 screws up some assumptions that airmash makes
    self.world.spawn_at(Entity::from_bits(0), ());

    for func in crate::HANDLERS {
      func(&self.dispatcher);
    }
  }

  pub fn shutdown(&self) {
    self.shutdown.store(true, Ordering::Relaxed);
  }

  pub fn register<E, H>(&mut self, handler: H)
  where
    E: Event,
    H: EventHandler<E>,
  {
    self.register_with_priority(crate::priority::DEFAULT, handler);
  }

  pub fn register_with_priority<E, H>(&mut self, priority: i32, handler: H)
  where
    E: Event,
    H: EventHandler<E>,
  {
    self.dispatcher.register_with_priority(priority, handler);
  }

  pub fn dispatch<E>(&mut self, event: E)
  where
    E: Event,
  {
    let dispatcher = self.dispatcher.clone();
    dispatcher.dispatch(event, self)
  }

  pub fn dispatch_many<I, E>(&mut self, events: I)
  where
    I: IntoIterator<Item = E>,
    E: Event,
  {
    let dispatcher = self.dispatcher.clone();
    for event in events {
      dispatcher.dispatch(event, self);
    }
  }
}

pub struct Resources {
  map: AnyMap,
}

impl Resources {
  pub fn new() -> Self {
    Self { map: AnyMap::new() }
  }

  pub fn read<T: 'static>(&self) -> Ref<T> {
    match self.get::<T>() {
      Some(val) => val,
      None => panic!(
        "Unable to access non-existant resource `{}`",
        std::any::type_name::<T>()
      ),
    }
  }

  pub fn write<T: 'static>(&self) -> RefMut<T> {
    match self.get_mut::<T>() {
      Some(val) => val,
      None => panic!(
        "Unable to access non-existant resource `{}`",
        std::any::type_name::<T>()
      ),
    }
  }

  pub fn get<T: 'static>(&self) -> Option<Ref<T>> {
    self.map.get::<RefCell<T>>().map(|x| x.borrow())
  }
  pub fn get_mut<T: 'static>(&self) -> Option<RefMut<T>> {
    self.map.get::<RefCell<T>>().map(|x| x.borrow_mut())
  }

  pub fn insert<T: 'static>(&mut self, value: T) -> Option<T> {
    self.map.insert(RefCell::new(value)).map(|x| x.into_inner())
  }
  pub fn remove<T: 'static>(&mut self) -> Option<T> {
    self.map.remove::<RefCell<T>>().map(|x| x.into_inner())
  }

  pub fn entry<T: 'static>(&mut self) -> anymap::Entry<dyn anymap::any::Any, T> {
    self.map.entry::<T>()
  }
}

impl Default for Resources {
  fn default() -> Self {
    Self::new()
  }
}

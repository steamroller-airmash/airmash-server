use crate::event::ServerStartup;
use crate::network::ConnectionMgr;
use crate::{dispatch::EventDispatcher, Event, EventHandler};
use anymap::AnyMap;
use hecs::Entity;
use std::cell::{Ref, RefCell, RefMut};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Main airmash game, containing all game data and resources.
pub struct AirmashGame {
  /// The world instance. Contains all the entities and their components.
  pub world: hecs::World,

  /// Resources. Essential a map based on type, contains anything that is not
  /// attached to an entity.
  pub resources: Resources,

  shutdown: Arc<AtomicBool>,
}

impl AirmashGame {
  /// Run a single iteration of the main game loop with the provided current
  /// time.
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

  /// Run the main game loop until the server is supposed to shut down.
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
  /// Create a new game with no default resources or event handlers.
  ///
  /// Usually you want [`with_network`] or [`with_test_defaults`] instead of
  /// this method.
  ///
  /// [`with_network`]: crate::AirmashGame::with_network
  /// [`with_test_defaults`]: crate::AirmashGame::with_test_defaults
  pub fn uninit() -> Self {
    let mut game = AirmashGame {
      world: hecs::World::new(),
      resources: Resources::new(),
      shutdown: Arc::new(AtomicBool::new(false)),
    };
    game.resources.insert(EventDispatcher::new());
    game
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

  /// Indicate that the server should shut down after the current frame.
  pub fn shutdown(&self) {
    self.shutdown.store(true, Ordering::Relaxed);
  }

  /// Register an event handler with the default priority.
  ///
  /// See the [`event`](crate::event) module docs for a description of how event
  /// handling works.
  pub fn register<E, H>(&mut self, handler: H)
  where
    E: Event,
    H: EventHandler<E>,
  {
    self.register_with_priority(crate::priority::DEFAULT, handler);
  }

  /// Register an event handler with a custom priority.
  ///
  /// See the [`event`](crate::event) module docs for a description of how event
  /// handling works.
  pub fn register_with_priority<E, H>(&mut self, priority: i32, handler: H)
  where
    E: Event,
    H: EventHandler<E>,
  {
    self.dispatcher().register_with_priority(priority, handler);
  }

  /// Dispatch an event and execute all the corresponding event handlers.
  ///
  /// If there is no event currently executing then the event will be dispatched
  /// immediately. However, if this is called while an event is currently being
  /// dispatched then the event will be queued up and dispatched once the
  /// current event has completed.
  ///
  /// See the [`event`](crate::event) module docs for a description of how event
  /// handling works.
  pub fn dispatch<E>(&mut self, event: E)
  where
    E: Event,
  {
    let dispatcher = self.dispatcher();
    dispatcher.dispatch(event, self)
  }

  /// Dispatch many events. This is functionally equivalent to calling
  /// [`dispatch`] in a loop and is provided for convenience.
  ///
  /// [`dispatch`]: crate::AirmashGame::dispatch
  pub fn dispatch_many<I, E>(&mut self, events: I)
  where
    I: IntoIterator<Item = E>,
    E: Event,
  {
    let dispatcher = self.dispatcher();
    for event in events {
      dispatcher.dispatch(event, self);
    }
  }
}

impl AirmashGame {
  pub(crate) fn dispatcher(&self) -> EventDispatcher {
    self.resources.read::<EventDispatcher>().clone()
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

    self.resources.insert(RegionName("default".to_owned()));
    self.resources.insert(GameType::FFA);

    // Having entities with id 0 screws up some assumptions that airmash makes
    self.world.spawn_at(Entity::from_bits(0), ());

    let dispatcher = self.dispatcher();
    for func in crate::dispatch::AIRMASH_EVENT_HANDLERS {
      func(&dispatcher);
    }
  }
}

/// Container for all resources used within the game.
///
/// Functionally, this is a map based on type. It also wraps all elements within
/// [`RefCell`]s so that multiple resources can be accessed at the same time.
pub struct Resources {
  map: AnyMap,
}

impl Resources {
  pub fn new() -> Self {
    Self { map: AnyMap::new() }
  }

  /// Attempt to access a resource `T` immutably.
  ///
  /// # Panics
  /// Panics if there is no resource with type `T` or if the resource is already
  /// borrowed mutably.
  pub fn read<T: 'static>(&self) -> Ref<T> {
    match self.get::<T>() {
      Some(val) => val,
      None => panic!(
        "Unable to access non-existant resource `{}`",
        std::any::type_name::<T>()
      ),
    }
  }

  /// Attempt to access a resource `T` mutably.
  ///
  /// # Panics
  /// Panics if there is no resource with type `T` or if the resource is already
  /// borrowed mutably.
  pub fn write<T: 'static>(&self) -> RefMut<T> {
    match self.get_mut::<T>() {
      Some(val) => val,
      None => panic!(
        "Unable to access non-existant resource `{}`",
        std::any::type_name::<T>()
      ),
    }
  }

  /// Attempt to access a resource `T` immutably.
  ///
  /// # Panics
  /// Panics if the resource is alread borrowed mutably.
  pub fn get<T: 'static>(&self) -> Option<Ref<T>> {
    self.map.get::<RefCell<T>>().map(|x| x.borrow())
  }

  /// Attempt to access a resource `T` mutably.
  ///
  /// # Panics
  /// Panics if the resource is alread borrowed mutably.
  pub fn get_mut<T: 'static>(&self) -> Option<RefMut<T>> {
    self.map.get::<RefCell<T>>().map(|x| x.borrow_mut())
  }

  /// Insert a new resource. Returns the old resource if one was already
  /// present.
  pub fn insert<T: 'static>(&mut self, value: T) -> Option<T> {
    self.map.insert(RefCell::new(value)).map(|x| x.into_inner())
  }

  /// Remove a resource. Returns the removed resource if it was removed.
  pub fn remove<T: 'static>(&mut self) -> Option<T> {
    self.map.remove::<RefCell<T>>().map(|x| x.into_inner())
  }

  /// Returns true if theis container has a resource of type `T`.
  pub fn contains<T: 'static>(&self) -> bool {
    self.map.contains::<RefCell<T>>()
  }

  /// Gets the entry for the given resource in the collection for in-place
  /// manipulation.
  pub fn entry<'a, T: 'static>(&'a mut self) -> ResourceEntry<'a, T> {
    ResourceEntry {
      entry: self.map.entry::<RefCell<T>>(),
    }
  }
}

/// A view into a single location in the [`Resources`] map.
pub struct ResourceEntry<'a, T> {
  entry: anymap::Entry<'a, dyn anymap::any::Any, RefCell<T>>,
}

impl<'a, T: 'static> ResourceEntry<'a, T> {
  /// Ensures that a value is in the entry by inserting the default if empty,
  /// and returns a mutable reference to the value in the entry.
  pub fn or_insert(self, default: T) -> &'a mut T {
    self.entry.or_insert(RefCell::new(default)).get_mut()
  }

  /// Ensures that a value is in the entry by inserting the result of the
  /// default function if empty, and returns a mutable reference to the value in
  /// the entry.
  pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> &'a mut T {
    self
      .entry
      .or_insert_with(move || RefCell::new(default()))
      .get_mut()
  }
}

impl Default for Resources {
  fn default() -> Self {
    Self::new()
  }
}

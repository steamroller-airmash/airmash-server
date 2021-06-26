use crate::component::IsPlayer;
use crate::event::ServerStartup;
use crate::network::{ConnectionId, ConnectionMgr};
use crate::protocol::{v5, ServerPacket, Team, Vector2};
use crate::{Event, EventDispatcher, EventHandler};
use anymap::AnyMap;
use hecs::Entity;
use std::cell::{Ref, RefCell, RefMut};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct AirmashWorld {
  pub world: hecs::World,
  pub resources: Resources,
  pub(crate) dispatcher: EventDispatcher,

  shutdown: Arc<AtomicBool>,
}

impl AirmashWorld {
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

impl AirmashWorld {
  pub fn uninit() -> Self {
    AirmashWorld {
      world: hecs::World::new(),
      resources: Resources::new(),
      dispatcher: EventDispatcher::new(),
      shutdown: Arc::new(AtomicBool::new(false)),
    }
  }

  pub fn with_network(addr: SocketAddr) -> Self {
    let mut me = Self::with_partial_defaults();
    me.resources
      .insert(ConnectionMgr::with_server(addr, me.shutdown.clone()));

    me
  }

  pub fn with_partial_defaults() -> Self {
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

  pub fn register_with_priority<E, H>(&mut self, priority: isize, handler: H)
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
}

impl AirmashWorld {
  pub fn send_to_conn(&self, conn: ConnectionId, packet: impl Into<ServerPacket>) {
    let mut connmgr = self.resources.write::<ConnectionMgr>();
    let data = match v5::serialize(&packet.into()) {
      Ok(data) => data,
      Err(_) => return,
    };

    connmgr.send_to_conn(conn, data);
  }

  pub fn send_to(&self, player: Entity, packet: impl Into<ServerPacket>) {
    self._send_to(player, &packet.into());
  }
  fn _send_to(&self, player: Entity, packet: &ServerPacket) {
    let mut connmgr = self.resources.write::<ConnectionMgr>();
    let data = match v5::serialize(packet) {
      Ok(data) => data,
      Err(_) => return,
    };

    connmgr.send_to(player, data);
  }

  pub fn send_to_visible(&self, pos: Vector2<f32>, packet: impl Into<ServerPacket>) {
    self._send_to_visible(pos, &packet.into());
  }
  fn _send_to_visible(&self, pos: Vector2<f32>, packet: &ServerPacket) {
    use crate::resource::collision::PlayerPosDb;
    use crate::resource::Config;

    let mut connmgr = self.resources.write::<ConnectionMgr>();
    let db = self.resources.read::<PlayerPosDb>();
    let config = self.resources.read::<Config>();

    let data = match v5::serialize(packet) {
      Ok(data) => data,
      Err(_) => return,
    };

    let mut entries = Vec::new();
    db.query(pos, config.view_radius, None, &mut entries);

    for entity in entries {
      connmgr.send_to(entity, data.clone());
    }
  }

  pub fn send_to_team(&self, team: Team, packet: impl Into<ServerPacket>) {
    self._send_to_team(team, &packet.into());
  }
  fn _send_to_team(&self, reqteam: Team, packet: &ServerPacket) {
    let mut connmgr = self.resources.write::<ConnectionMgr>();
    let mut query = self
      .world
      .query::<&crate::component::Team>()
      .with::<IsPlayer>();

    let data = match v5::serialize(packet) {
      Ok(data) => data,
      Err(_) => return,
    };

    for (ent, team) in query.iter() {
      if team.0 != reqteam {
        continue;
      }

      connmgr.send_to(ent, data.clone());
    }
  }

  pub fn send_to_team_visible(
    &self,
    team: Team,
    pos: Vector2<f32>,
    packet: impl Into<ServerPacket>,
  ) {
    self._send_to_team_visible(team, pos, &packet.into());
  }
  fn _send_to_team_visible(&self, team: Team, pos: Vector2<f32>, packet: &ServerPacket) {
    use crate::resource::collision::PlayerPosDb;
    use crate::resource::Config;

    let mut connmgr = self.resources.write::<ConnectionMgr>();
    let db = self.resources.read::<PlayerPosDb>();
    let config = self.resources.read::<Config>();

    let data = match v5::serialize(packet) {
      Ok(data) => data,
      Err(_) => return,
    };

    let mut entries = Vec::new();
    db.query(pos, config.view_radius, Some(team), &mut entries);

    for entity in entries {
      connmgr.send_to(entity, data.clone());
    }
  }

  pub fn send_to_all(&self, packet: impl Into<ServerPacket>) {
    self._send_to_all(&packet.into());
  }
  pub fn _send_to_all(&self, packet: &ServerPacket) {
    let mut connmgr = self.resources.write::<ConnectionMgr>();
    let mut query = self.world.query::<()>().with::<IsPlayer>();

    let data = match v5::serialize(packet) {
      Ok(data) => data,
      Err(_) => return,
    };

    for (ent, ..) in query.iter() {
      connmgr.send_to(ent, data.clone());
    }
  }

  pub fn send_to_others(&self, player: Entity, packet: impl Into<ServerPacket>) {
    self._send_to_others(player, &packet.into());
  }
  fn _send_to_others(&self, player: Entity, packet: &ServerPacket) {
    let mut connmgr = self.resources.write::<ConnectionMgr>();
    let mut query = self.world.query::<()>().with::<IsPlayer>();

    let data = match v5::serialize(packet) {
      Ok(data) => data,
      Err(_) => return,
    };

    for (ent, ..) in query.iter() {
      if ent == player {
        continue;
      }

      connmgr.send_to(ent, data.clone());
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
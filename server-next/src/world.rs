use crate::component::IsPlayer;
use crate::network::ConnectionMgr;
use crate::protocol::{v5::serialize, ServerPacket};
use crate::{Event, EventDispatcher, EventHandler};
use airmash_protocol::{v5, Team, Vector2};
use anymap::AnyMap;
use hecs::Entity;
use std::cell::{Ref, RefCell, RefMut};
use std::time::Instant;

pub struct AirmashWorld {
  pub world: hecs::World,
  pub resources: Resources,
  dispatcher: EventDispatcher,
}

impl AirmashWorld {
  pub fn uninit() -> Self {
    AirmashWorld {
      world: hecs::World::new(),
      resources: Resources::new(),
      dispatcher: EventDispatcher::new(),
    }
  }

  pub fn with_defaults() -> Self {
    let mut me = Self::uninit();
    me.init_defaults();
    me
  }

  fn init_defaults(&mut self) {
    use crate::resource::*;

    let now = Instant::now();

    self.resources.insert(StartTime(now));
    self.resources.insert(LastFrame(now));
    self.resources.insert(ThisFrame(now));
    self.resources.insert(Config::default());

    for func in crate::HANDLERS {
      func(&self.dispatcher);
    }
  }

  pub fn register<E, H>(&mut self, handler: H)
  where
    E: Event,
    H: EventHandler<E>,
  {
    self.dispatcher.register(handler);
  }

  pub fn dispatch<E>(&mut self, event: E)
  where
    E: Event,
  {
    let dispatcher = self.dispatcher.clone();
    dispatcher.dispatch(event, self)
  }
}

#[allow(unused_variables)]
impl AirmashWorld {
  pub fn send_to(&self, player: Entity, packet: impl Into<ServerPacket>) {
    self._send_to(player, &packet.into());
  }
  fn _send_to(&self, player: Entity, packet: &ServerPacket) {
    let mut connmgr = self.resources.write::<ConnectionMgr>();
    let data = match v5::serialize(packet.into()) {
      Ok(data) => data,
      Err(_) => return,
    };

    connmgr.send_to(player, data);
  }

  pub fn send_to_visible(&self, pos: Vector2<f32>, packet: impl Into<ServerPacket>) {
    unimplemented!()
  }

  pub fn send_to_team(&self, team: Team, packet: impl Into<ServerPacket>) {
    self._send_to_team(team, &packet.into());
  }
  fn _send_to_team(&self, reqteam: Team, packet: &ServerPacket) {
    let mut connmgr = self.resources.write::<ConnectionMgr>();
    let mut query = self
      .world
      .query::<&crate::component::Team>()
      .with::<&IsPlayer>();

    let data = match v5::serialize(packet.into()) {
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
    unimplemented!()
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
}

impl Default for Resources {
  fn default() -> Self {
    Self::new()
  }
}

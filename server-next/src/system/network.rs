use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::SocketAddrV4;
use std::str::FromStr;

use airmash_protocol::client::{self as c, Login};
use airmash_protocol::ClientPacket;
use airmash_protocol::FlagCode;
use airmash_protocol::PlaneType;
use bstr::BString;
use hecs::EntityBuilder;
use uuid::Uuid;

use crate::component::IsPlayer;
use crate::event::*;
use crate::protocol::v5::deserialize;
use crate::resource::Config;
use crate::resource::TakenNames;
use crate::{network::*, AirmashWorld};

pub fn process_packets(game: &mut AirmashWorld) {
  loop {
    let mut conn_mgr = game.resources.write::<ConnectionMgr>();
    let (conn, evt) = match conn_mgr.next_packet() {
      Some(evt) => evt,
      None => return,
    };

    if let ConnectionEvent::Opened = &evt {
      debug!(
        "Got new connection from {} with id {:?}",
        conn_mgr
          .socket_addr(conn)
          .unwrap_or(SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(0, 0, 0, 0),
            0000
          ))),
        conn
      );
    }

    let assoc = conn_mgr.associated(conn);

    drop(conn_mgr);

    let data = match evt {
      ConnectionEvent::Opened => {
        continue;
      }
      ConnectionEvent::Data(data) => data,
      ConnectionEvent::Closed(None) => continue,
      ConnectionEvent::Closed(Some(entity)) => {
        if !game.world.contains(entity) {
          continue;
        }

        if !game.world.get_mut::<&IsPlayer>(entity).is_ok() {
          warn!(
            "Connection {:?} was for a non player entity {:?}",
            conn, entity
          );
          continue;
        }

        game.dispatch(PlayerLeave { player: entity });

        continue;
      }
    };

    let packet = match deserialize::<ClientPacket>(&data) {
      Ok(packet) => packet,
      Err(_) => {
        debug!("Dropping malformed packet from {:?}", conn);
        continue;
      }
    };

    // Other packets are only valid once the connection has been initiated.
    if assoc.is_none() && !matches!(packet, ClientPacket::Login(_) | ClientPacket::Backup(_)) {
      continue;
    }

    match packet {
      ClientPacket::Login(login) => handle_login(game, login, conn),
      ClientPacket::Backup(_) => continue,
      ClientPacket::Horizon(packet) => game.dispatch(PacketEvent {
        entity: assoc.unwrap(),
        conn,
        packet,
      }),
      ClientPacket::Ack => game.dispatch(PacketEvent {
        entity: assoc.unwrap(),
        conn,
        packet: c::Ack,
      }),
      ClientPacket::Pong(packet) => game.dispatch(PacketEvent {
        entity: assoc.unwrap(),
        conn,
        packet,
      }),
      ClientPacket::Key(packet) => game.dispatch(PacketEvent {
        entity: assoc.unwrap(),
        conn,
        packet,
      }),
      ClientPacket::Command(packet) => game.dispatch(PacketEvent {
        entity: assoc.unwrap(),
        conn,
        packet,
      }),
      ClientPacket::ScoreDetailed => game.dispatch(PacketEvent {
        entity: assoc.unwrap(),
        conn,
        packet: c::ScoreDetailed,
      }),
      ClientPacket::Chat(packet) => game.dispatch(PacketEvent {
        entity: assoc.unwrap(),
        conn,
        packet,
      }),
      ClientPacket::TeamChat(packet) => game.dispatch(PacketEvent {
        entity: assoc.unwrap(),
        conn,
        packet,
      }),
      ClientPacket::Whisper(packet) => game.dispatch(PacketEvent {
        entity: assoc.unwrap(),
        conn,
        packet,
      }),
      ClientPacket::Say(packet) => game.dispatch(PacketEvent {
        entity: assoc.unwrap(),
        conn,
        packet,
      }),
      ClientPacket::VoteMute(packet) => game.dispatch(PacketEvent {
        entity: assoc.unwrap(),
        conn,
        packet,
      }),
      ClientPacket::LocalPing(packet) => game.dispatch(PacketEvent {
        entity: assoc.unwrap(),
        conn,
        packet,
      }),
    }
  }
}

fn make_unique_name(names: &mut TakenNames, name: &mut BString) {
  'outer: while names.contains(name) {
    let mut ext = 0;
    for _ in 0..100 {
      ext = rand::random::<u32>() % 1000;

      name.append(&mut format!("#{:03}", ext).into_bytes());
      if !names.contains(name) {
        break 'outer;
      }
      let len = name.len();
      name.truncate(len - 4);
    }

    name.append(&mut format!("#{:03}", ext).into_bytes());
  }

  name.truncate(255);
  names.insert(name.clone());
}

fn handle_login(game: &mut AirmashWorld, mut login: Login, conn: ConnectionId) {
  use crate::component::*;
  use crate::protocol::{server as s, Vector2};
  use crate::resource::{EntityMapping, StartTime};

  debug!("Handling login on {}", conn);

  let entity = {
    let config = game.resources.read::<Config>();
    let info = &config.planes[PlaneType::Predator];

    let mut conn_mgr = game.resources.write::<ConnectionMgr>();
    let mut names = game.resources.write::<TakenNames>();
    let mut mapping = game.resources.write::<EntityMapping>();
    let start_time = game.resources.read::<StartTime>().0;

    if login.protocol != 5 {
      game.send_to_conn(
        conn,
        s::Error {
          error: airmash_protocol::ErrorType::IncorrectProtocolLevel,
        },
      );
      return;
    }

    if login.name.len() > 40 {
      game.send_to_conn(
        conn,
        s::Error {
          error: airmash_protocol::ErrorType::InvalidLogin,
        },
      );
      return;
    }

    make_unique_name(&mut names, &mut login.name);

    let mut builder = EntityBuilder::new();
    builder
      .add(IsPlayer)
      .add(Position(Vector2::zeros()))
      .add(Velocity(Vector2::zeros()))
      .add(Rotation(0.0))
      .add(Energy(1.0))
      .add(Health(1.0))
      .add(EnergyRegen(info.energy_regen))
      .add(HealthRegen(info.health_regen))
      .add(PlaneType::Predator)
      .add(FlagCode::from_str(&login.flag.to_string()).unwrap_or(FlagCode::UnitedNations))
      .add(Level(0))
      .add(Score(0))
      .add(KillCount(0))
      .add(DeathCount(0))
      .add(Upgrades::default())
      .add(Name(login.name.clone()))
      .add(Team(0))
      .add(IsAlive(true))
      .add(Session(Uuid::new_v4()))
      .add(KeyState::default())
      .add(LastFireTime(start_time))
      .add(SpecialActive(false))
      .add(Powerup::default());

    let entity = game.world.spawn(builder.build());

    if entity.id() > u16::MAX as _ {
      game.send_to_conn(
        conn,
        s::Error {
          error: airmash_protocol::ErrorType::UnknownError,
        },
      );
      let _ = game.world.despawn(entity);

      return;
    }

    debug!(
      "Player {} with id {:?} login on connection {}",
      login.name, entity, conn
    );

    conn_mgr.associate(entity, conn);
    mapping.insert(entity.id() as u16, entity);

    entity
  };

  game.dispatch(EntitySpawn { entity });
  game.dispatch(PlayerJoin { player: entity });
}

use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::SocketAddrV4;
use std::str::FromStr;

use airmash_protocol::client::{self as c, Login};
use airmash_protocol::ClientPacket;
use airmash_protocol::FlagCode;
use airmash_protocol::PlaneType;
use hecs::EntityBuilder;

use crate::component::IsPlayer;
use crate::event::*;
use crate::protocol::v5::deserialize;
use crate::resource::Config;
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

fn handle_login(game: &mut AirmashWorld, login: Login, conn: ConnectionId) {
  use crate::component::*;
  use crate::protocol::Vector2;

  let config = game.resources.read::<Config>();
  let mut conn_mgr = game.resources.write::<ConnectionMgr>();
  let info = &config.planes[PlaneType::Predator];

  let mut builder = EntityBuilder::new();
  builder
    .add(Position(Vector2::zeros()))
    .add(Velocity(Vector2::zeros()))
    .add(Rotation(0.0))
    .add(Energy(1.0))
    .add(Health(1.0))
    .add(EnergyRegen(info.energy_regen))
    .add(HealthRegen(info.health_regen))
    .add(PlaneType::Predator)
    .add(FlagCode::from_str(&login.flag.to_string()).unwrap_or(FlagCode::UnitedNations))
    .add(IsPlayer)
    .add(Upgrades::default())
    .add(IsAlive);

  let entity = game.world.spawn(builder.build());

  conn_mgr.associate(entity, conn);

  drop(conn_mgr);
  drop(config);

  game.dispatch(PlayerJoin { player: entity });
}

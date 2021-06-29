use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::SocketAddrV4;

use bstr::BString;

use crate::component::IsPlayer;
use crate::event::*;
use crate::protocol::client::{self as c, Login};
use crate::protocol::v5::deserialize;
use crate::protocol::ClientPacket;
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

        if !game.world.get_mut::<IsPlayer>(entity).is_ok() {
          warn!(
            "Connection {:?} was for a non player entity {:?}",
            conn, entity
          );
          continue;
        }

        game.dispatch(PlayerLeave { player: entity });
        game.despawn(entity);

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
  use crate::protocol::server as s;
  use crate::resource::{EntityMapping, StartTime, ThisFrame};

  debug!("Handling login on {}", conn);

  let entity = {
    let config = game.resources.read::<Config>();

    let mut conn_mgr = game.resources.write::<ConnectionMgr>();
    let mut names = game.resources.write::<TakenNames>();
    let mut mapping = game.resources.write::<EntityMapping>();
    let start_time = game.resources.read::<StartTime>().0;
    let this_frame = game.resources.read::<ThisFrame>().0;

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

    let mut builder =
      crate::defaults::build_default_player(&login, &config, start_time, this_frame);
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

    game.world.get_mut::<Team>(entity).unwrap().0 = entity.id() as _;

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

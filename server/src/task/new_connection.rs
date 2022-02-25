use super::TaskData;

use crate::component::channel::{OnClose, OnLogin, OnLoginReader, OnPlayerJoin};
use crate::component::collection::PlayerNames;
use crate::component::counter::*;
use crate::component::event::{ConnectionClose, PlayerJoin};
use crate::component::flag::*;
use crate::component::ratelimit::*;
use crate::component::time::*;
use crate::consts::{throttling::*, NUM_PLAYERS};
use crate::types::*;

use crate::protocol::client::Login;
use crate::protocol::{FlagCode, PlayerStatus};

use std::convert::TryFrom;
use std::str::FromStr;
use std::time::{Duration, Instant};

use specs::world::Builder;
use uuid::Uuid;

/// Wait for the login packet and, if it arrives, perform
/// all the state setup required.
///
/// It is possible for the login packet to arrive before
/// the reader was created so this task also has an
/// optional parameter to pass in the login packet if
/// it has already been received.
pub async fn new_connection(
  mut task: TaskData,
  id: ConnectionId,
  mut reader: OnLoginReader,
  packet: Option<Login>,
) {
  // If the client doesn't respond within a fixed time
  // we'll drop the connection
  let timeout = Instant::now() + Duration::from_secs(10);

  // Wait for a login packet or timeout
  // TODO: Backup packets?
  let packet = if packet.is_some() {
    packet
  } else {
    loop {
      let read = task.read_resource::<OnLogin, _, _>(|channel| {
        channel
          .read(&mut reader)
          .filter(|(conn, _)| *conn == id)
          .map(|(_, packet)| packet.clone())
          .next()
      });

      if let Some(packet) = read {
        break Some(packet);
      }

      if Instant::now() > timeout {
        break None;
      }

      task.yield_frame().await;
    }
  };

  if let Some(login) = packet {
    do_login(&mut task, id, login);
  } else {
    // TODO: Figure out if this is the right way to do this
    task.write_resource::<OnClose, _, _>(|mut on_close| {
      on_close.single_write(ConnectionClose { conn: id });
    });
  }
}

fn do_login(task: &mut TaskData, conn: ConnectionId, login: Login) {
  let flag = FlagCode::try_from(&*login.flag).unwrap_or(FlagCode::UnitedNations);
  let session = Uuid::from_str(&login.session).ok();

  let this_frame = task.read_resource::<ThisFrame, _, _>(|r| r.0);
  let start_time = task.read_resource::<StartTime, _, _>(|r| r.0);

  let name = task.read_resource::<PlayerNames, _, _>(|names| {
    use rand::distributions::{IndependentSample, Range};
    let range = Range::new(0, 1000);
    let mut rng = rand::thread_rng();
    let mut name = login.name;

    while names.0.contains(&name) {
      name = format!("{}#{:03}", name, range.ind_sample(&mut rng));
    }

    name
  });
  let mut trunc_name = name.clone();
  trunc_name.truncate(255);
  trunc_name.shrink_to_fit();

  let entity = task.create_entity(|builder| {
    builder
      .with(Energy::new(1.0))
      .with(Health::new(1.0))
      .with(KeyState::default())
      .with(Upgrades::default())
      .with(Rotation::default())
      .with(Velocity::default())
      .with(Level(0))
      .with(Score(0))
      .with(Earnings(Score(0)))
      .with(JoinTime(this_frame))
      .with(TotalKills(0))
      .with(TotalDeaths(0))
      .with(LastRepelTime(this_frame))
      .with(ChatMuteLimiter(RateLimiter::new(MUTE_LIMIT, MUTE_PERIOD)))
      .with(ChatThrottleLimiter(RateLimiter::new(
        THROTTLE_LIMIT,
        THROTTLE_PERIOD,
      )))
      .with(Name(trunc_name.clone()))
      .with(Session(session))
      .with(LastStealthTime(start_time))
      .with(PlayerStatus::Alive)
      .with(flag)
      .with(LastUpdate(this_frame))
      .with(IsPlayer)
      .with(PingData::default())
      .with(LastShotTime(start_time))
      .with(LastKeyTime(this_frame))
      .build()
  });

  if entity.id() > 0xFFFF {
    error!(
      target: "server",
      "Entity created with id greater than 0xFFFF. Aborting to avoid sending invalid packets."
    );
    panic!("Entity created with invalid id.");
  }

  let (team, plane, pos) = task.world(|world| {
    let mut game: GameModeWriter<dyn GameMode> = world.system_data();
    let game = game.get_mut();
    let team = game.assign_team(entity);
    let plane = game.assign_plane(entity, team);
    let pos = game.spawn_pos(entity, team);

    (team, plane, pos)
  });

  let (energy_regen, health_regen) = task.read_resource::<Config, _, _>(|config| {
    let ref info = config.planes[plane];
    (info.energy_regen, info.health_regen)
  });

  // Components that required knowing the entity to create
  task.insert(entity, team).unwrap();
  task.insert(entity, plane).unwrap();
  task.insert(entity, pos).unwrap();
  task.insert(entity, energy_regen).unwrap();
  task.insert(entity, health_regen).unwrap();

  NUM_PLAYERS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
  task.write_resource::<PlayersGame, _, _>(|mut playersgame| {
    playersgame.0 += 1;
  });

  task.write_resource::<PlayerNames, _, _>(move |mut names| {
    names.0.insert(name, entity);
  });

  task.write_resource::<OnPlayerJoin, _, _>(move |mut on_join| {
    on_join.single_write(PlayerJoin {
      id: entity,
      plane: plane,
      team: team,
      level: Level(0),
      name: Name(trunc_name),
      flag: flag,
      session: Session(session),
      conn: conn,
    });
  });
}

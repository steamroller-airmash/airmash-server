use crate::component::*;
use crate::component::{counter::*, flag::*, time::*};
use crate::ecs::{prelude::*, ReadAdapter, SystemData, World};
use crate::event::PlayerJoin;
use crate::protocol::client::Login;
use crate::resource::{
    builtin::{CurrentFrame, StartTime},
    channel::OnPlayerJoin,
    packet::{ClientPacket, OnLogin},
    socket::{ConnectEvent, OnConnect, SocketId},
    Config, Connections, PlayerNames,
};
use crate::sysdata::{ConnectionsNoTeams, GameModeWriter, TaskData, TaskSpawner};
use crate::util::{GameMode, MaybeInit};
use crate::*;

use futures::{select, FutureExt};
use shrev::ReaderId;
use tokio::time::Duration;
use uuid::Uuid;

use std::convert::TryInto;
use std::str::FromStr;

#[derive(Default)]
struct HandleConnect {
    login: MaybeInit<ReaderId<ClientPacket<Login<'static>>>>,
    connect: MaybeInit<ReaderId<ConnectEvent>>,
}

impl HandleConnect {
    fn setup(&mut self, world: &mut World) {
        ConnectTaskData::setup(world);

        self.login = MaybeInit::new(world.fetch_resource_mut::<OnLogin>().register_reader());
        self.connect = MaybeInit::new(world.fetch_resource_mut::<OnConnect>().register_reader());
    }
}

#[system(state = HandleConnect)]
fn handle_connect<'a>(
    state: &mut HandleConnect,
    connects: &mut Read<'a, OnConnect>,
    logins: &mut Write<'a, OnLogin, ReadAdapter<OnLogin>>,
    conns: &mut Write<'a, Connections>,
    tasks: &TaskSpawner<'a>,
) {
    for evt in connects.read(&mut state.connect) {
        debug!("New connection opened with id {}", evt.socketid);

        conns.register_new(evt.socketid, evt.socket.clone());

        let reader = logins.duplicate_reader(&state.login);
        let socket = evt.socketid;
        tasks.spawn(move |world| new_connection(world, reader, socket));
    }

    // Advance login channel reader to the end.
    logins.read(&mut state.login);
}

async fn wait_for_login(
    data: &mut TaskData,
    reader: &mut ReaderId<ClientPacket<Login<'static>>>,
    socket: SocketId,
) -> Login<'static> {
    loop {
        let packet = data.world(|world| {
            let channel: Read<OnLogin> = world.system_data();

            channel
                .read(reader)
                .filter(|packet| packet.connection == socket)
                .map(|packet| packet.packet.clone())
                .next()
        });

        if let Some(packet) = packet {
            break packet;
        }

        data.yield_frame().await;
    }
}

async fn new_connection(
    mut data: TaskData,
    mut reader: ReaderId<ClientPacket<Login<'static>>>,
    socket: SocketId,
) {
    let wait_time = Duration::from_secs(10);

    trace!("Starting login task for {}", socket);

    select! {
        _ = tokio::time::delay_for(wait_time).fuse() => {
            data.world(|world| {
                let mut conns: Write<Connections> = world.system_data();
                let _ = conns.close(socket);
            });

            debug!("Client on socket {} failed to login", socket);
        },
        login = wait_for_login(&mut data, &mut reader, socket).fuse() => {
            trace!("Received Login packet from {}", socket);
            do_login(&mut data, socket, login);
        }
    }
}

fn do_login(data: &mut TaskData, conn: SocketId, login: Login) {
    let flag: Flag = (*login.flag).try_into().unwrap_or(Flag::UnitedNations);
    let session = Uuid::from_str(&login.session).ok();

    let CurrentFrame(this_frame) = data.read_resource(|r| *r);
    let StartTime(start_time) = data.read_resource(|r| *r);

    let name = &login.name;
    let name = data.write_resource::<PlayerNames, _, _>(|names| {
        use rand::distributions::{Distribution, Uniform};
        use std::fmt::Write;

        let dist = Uniform::from(0..1000);
        let mut rng = rand::thread_rng();
        let mut name = name.to_string();

        if !names.0.contains_key(&*name) {
            return name;
        }

        loop {
            let current_name = name.clone();

            for _ in 0..10 {
                name.clear();
                let _ = write!(&mut name, "{}#{:03}", current_name, dist.sample(&mut rng));

                if !names.0.contains_key(&name) {
                    return name;
                }
            }
        }
    });

    let mut trunc_name = name.clone();
    trunc_name.truncate(255);
    trunc_name.shrink_to_fit();

    let entity = data.create_entity(|builder| {
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
            .with(Name(trunc_name.clone()))
            .with(Session(session))
            .with(LastStealthTime(start_time))
            .with(PlayerStatus::Alive)
            .with(flag)
            .with(IsPlayer)
            .with(LastShotTime(start_time))
            .with(LastKeyTime(this_frame))
            .with(AssociatedConnection(conn))
            .with(LastUpdate(start_time))
            .build()
    });

    if entity.id() > 0xFFFF {
        error!(
            target: "server",
            "Entity created with id greater than 0xFFFF. Aborting to avoid sending invalid packets."
        );
        panic!("Entity created with invalid id.");
    }

    data.write_resource::<PlayerNames, _, _>(|mut names| {
        names.0.insert(name, entity);
    });

    let (team, plane, pos) = data.world(|world| {
        let mut gamemode: GameModeWriter<dyn GameMode> = world.system_data();
        let gamemode = gamemode.get_mut();

        let team = gamemode.assign_team(entity);
        let plane = gamemode.assign_plane(entity, team);
        let pos = gamemode.spawn_pos(entity, team);

        (team, plane, pos)
    });

    data.world(|world| {
        let config: Read<Config> = world.system_data();
        let mut energy_regen: WriteStorage<EnergyRegen> = world.system_data();
        let mut health_regen: WriteStorage<HealthRegen> = world.system_data();

        energy_regen
            .insert(entity, config.planes[plane].energy_regen)
            .unwrap();
        health_regen
            .insert(entity, config.planes[plane].health_regen)
            .unwrap();
    });

    let res = data.world(|world| {
        let mut teams: WriteStorage<Team> = world.system_data();
        let mut planes: WriteStorage<Plane> = world.system_data();
        let mut positions: WriteStorage<Position> = world.system_data();

        teams.insert(entity, team).unwrap();
        planes.insert(entity, plane).unwrap();
        positions.insert(entity, pos).unwrap();

        let mut conns: ConnectionsNoTeams = world.system_data();
        if let Err(_) = conns.associate(conn, entity) {
            warn!(
                "Socket {:?} disappeared during login for player {:?}",
                conn, entity
            );

            // Delete the entity since login failed
            let entities: Entities = world.system_data();
            let _ = entities.delete(entity);

            return false;
        }

        true
    });

    if !res {
        return;
    }

    info!("Player '{}' joined as {:?}", trunc_name, entity);

    data.write_resource::<OnPlayerJoin, _, _>(move |mut channel| {
        channel.single_write(PlayerJoin {
            id: entity,
            name: Name(trunc_name),
            level: Level(0),
            session: Session(session),
            conn,
            plane,
            team,
            flag,
        });
    });
}

/// Data types used by the connect task
///
/// This struct is never used and is instead an
/// easy shortcut to setup all the required storages
/// and resources.
#[derive(SystemData)]
#[allow(dead_code)]
struct ConnectTaskData<'a> {
    energy: ReadStorage<'a, Energy>,
    health: ReadStorage<'a, Health>,
    keystate: ReadStorage<'a, KeyState>,
    upgrades: ReadStorage<'a, Upgrades>,
    rotation: ReadStorage<'a, Rotation>,
    velocity: ReadStorage<'a, Velocity>,

    energy_regen: ReadStorage<'a, EnergyRegen>,
    health_regen: ReadStorage<'a, HealthRegen>,

    level: ReadStorage<'a, Level>,
    score: ReadStorage<'a, Score>,
    earnings: ReadStorage<'a, Earnings>,
    join_time: ReadStorage<'a, JoinTime>,
    total_kills: ReadStorage<'a, TotalKills>,
    total_deaths: ReadStorage<'a, TotalDeaths>,
    last_repel_time: ReadStorage<'a, LastRepelTime>,
    name: ReadStorage<'a, Name>,
    session: ReadStorage<'a, Session>,
    last_stealth_time: ReadStorage<'a, LastStealthTime>,
    status: ReadStorage<'a, PlayerStatus>,
    flag: ReadStorage<'a, Flag>,
    is_player: ReadStorage<'a, IsPlayer>,
    last_shot_time: ReadStorage<'a, LastShotTime>,
    last_key_time: ReadStorage<'a, LastKeyTime>,
    associated: ReadStorage<'a, AssociatedConnection>,
    team: ReadStorage<'a, Team>,
    plane: ReadStorage<'a, Plane>,
    position: ReadStorage<'a, Position>,

    player_names: Read<'a, PlayerNames>,
    player_join: Write<'a, OnPlayerJoin>,
}

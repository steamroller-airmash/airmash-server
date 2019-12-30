use crate::component::{
    counter::*, flag::*, time::*, AssociatedConnection, KeyState, Name, Session,
};
use crate::ecs::prelude::*;
use crate::ecs::{ReadAdapter, World};
use crate::event::PlayerJoin;
use crate::protocol::client::Login;
use crate::protocol::*;
use crate::resource::builtin::{CurrentFrame, StartTime};
use crate::resource::channel::OnPlayerJoin;
use crate::resource::packet::{ClientPacket, OnLogin};
use crate::resource::socket::{ConnectEvent, OnConnect, SocketId};
use crate::resource::{Connections, PlayerNames};
use crate::sysdata::{GameModeWriter, TaskData, TaskSpawner};
use crate::util::{GameMode, MaybeInit};

use futures::{select, FutureExt};
use shrev::ReaderId;
use tokio::time::Duration;
use uuid::Uuid;

use std::convert::TryInto;
use std::str::FromStr;

#[derive(Default)]
struct HandleConnect {
    login: MaybeInit<ReaderId<ClientPacket<Login>>>,
    connect: MaybeInit<ReaderId<ConnectEvent>>,
}

impl HandleConnect {
    fn setup(&mut self, world: &mut World) {
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
    reader: &mut ReaderId<ClientPacket<Login>>,
    socket: SocketId,
) -> Login {
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
    mut reader: ReaderId<ClientPacket<Login>>,
    socket: SocketId,
) {
    let wait_time = Duration::from_secs(10);

    select! {
        _ = tokio::time::delay_for(wait_time).fuse() => {
            data.world(|world| {
                let mut conns: Write<Connections> = world.system_data();
                let _ = conns.close(socket);
            });
        },
        login = wait_for_login(&mut data, &mut reader, socket).fuse() => {
            do_login(&mut data, socket, login);
        }
    }
}

fn do_login(data: &mut TaskData, conn: SocketId, login: Login) {
    let flag: FlagCode = login.flag.try_into().unwrap_or(FlagCode::UnitedNations);
    let session = Uuid::from_str(&login.session).ok();

    let CurrentFrame(this_frame) = data.read_resource(|r| *r);
    let StartTime(start_time) = data.read_resource(|r| *r);

    let name = &login.name;
    let name = data.write_resource::<PlayerNames, _, _>(|names| {
        use rand::distributions::{Distribution, Uniform};
        use std::fmt::Write;

        let dist = Uniform::from(0..1000);
        let mut rng = rand::thread_rng();
        let mut name = name.clone();

        if !names.0.contains_key(&name) {
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

    data.write_storage::<Team, _, _>(|mut res| {
        res.insert(entity, team).unwrap();
    });
    data.write_storage::<PlaneType, _, _>(|mut res| {
        res.insert(entity, plane).unwrap();
    });
    data.write_storage::<Position, _, _>(|mut res| res.insert(entity, pos).unwrap());

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

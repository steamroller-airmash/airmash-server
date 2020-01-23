use crate::component::{flag::IsPlayer, AssociatedConnection, Team};
use crate::ecs::{prelude::*, SystemData};
use crate::protocol::{Protocol, ServerPacket};
use crate::resource::{
    collision::{HitCircle, PlayerGrid},
    socket::SocketId,
    Config, Connections as ConnData, NonexistantSocketError,
};
use crate::util::RcBuf;
use crate::Position;

use std::ops::{Deref, DerefMut};

use airmash_protocol_v5::{ProtocolV5, SerializeError};

pub type Connections<'a> = ConnsInternal<'a, ReadStorage<'a, Team>, Read<'a, ConnData>>;
pub type ConnectionsMut<'a> = ConnsInternal<'a, ReadStorage<'a, Team>, Write<'a, ConnData>>;
pub type ConnectionsNoTeams<'a> = ConnsInternal<'a, (), Write<'a, ConnData>>;

#[derive(SystemData)]
pub struct ConnsInternal<'a, Teams, Conns> {
    config: ReadExpect<'a, Config>,
    entities: Entities<'a>,
    grid: Read<'a, PlayerGrid>,
    associated: ReadStorage<'a, AssociatedConnection>,

    is_player: ReadStorage<'a, IsPlayer>,

    team: Teams,
    conns: Conns,
}

pub struct SendIter<It, Ref>
where
    It: Iterator<Item = Entity>,
{
    conns: Ref,
    iter: It,
}

impl<'a, 'b: 'a, It, Teams, Conns> SendIter<It, &'b ConnsInternal<'a, Teams, Conns>>
where
    It: Iterator<Item = Entity> + 'b,
{
    fn new(conns: &'b ConnsInternal<'a, Teams, Conns>, iter: It) -> Self {
        Self { conns, iter }
    }
}

impl<'a, 'b: 'a, It, Teams, Conns> SendIter<It, &'b ConnsInternal<'a, Teams, Conns>>
where
    It: Iterator<Item = Entity> + 'b,
    Conns: Deref<Target = ConnData>,
{
    /// Exclude the given player from those being sent messages
    pub fn except(
        self,
        player: Entity,
    ) -> SendIter<impl Iterator<Item = Entity> + 'b, &'b ConnsInternal<'a, Teams, Conns>> {
        let iter = self.iter.filter(move |&ent| ent != player);
        SendIter::new(self.conns, iter)
    }

    /// Send the messages to all players that match the conditions
    pub fn send_all<I>(self, msg: I)
    where
        I: Into<ServerPacket>,
    {
        self.send_all_ref(&msg.into());
    }

    /// Send the messages to all players that match the conditions
    pub fn send_all_ref(self, msg: &ServerPacket) {
        use arrayvec::ArrayVec;

        let Self { iter, conns } = self;
        let iter = iter
            .map(|ent| ent)
            .filter_map(|ent| conns.associated.get(ent));

        let encoded: ArrayVec<[_; 8]> = match encode_message(msg) {
            Ok(encoded) => encoded.collect(),
            Err(e) => {
                warn!("Tried to serialize unencodable packet error: {}", e);
                return;
            }
        };

        for assoc in iter {
            for buf in encoded.iter().cloned() {
                if let Err(e) = conns.conns.send_to(assoc.0, buf) {
                    warn!("Unable to send to connection {:?}: {}", assoc.0, e);
                    break;
                }
            }
        }
    }
}

impl<'a, 'b: 'a, It, Conns> SendIter<It, &'b ConnsInternal<'a, ReadStorage<'a, Team>, Conns>>
where
    It: Iterator<Item = Entity> + 'b,
{
    /// Only send the message to players on the given team
    pub fn on_team(
        self,
        team: Team,
    ) -> SendIter<
        impl Iterator<Item = Entity> + 'b,
        &'b ConnsInternal<'a, ReadStorage<'a, Team>, Conns>,
    > {
        let conns = self.conns;
        let iter = self
            .iter
            .filter(move |&ent| conns.team.get(ent).map(|&x| x == team).unwrap_or(false));

        SendIter::new(conns, iter)
    }

    /// Only send the message to players not on the given team
    pub fn not_on_team(
        self,
        team: Team,
    ) -> SendIter<
        impl Iterator<Item = Entity> + 'b,
        &'b ConnsInternal<'a, ReadStorage<'a, Team>, Conns>,
    > {
        let conns = self.conns;
        let iter = self
            .iter
            .filter(move |&ent| conns.team.get(ent).map(|&x| x != team).unwrap_or(false));

        SendIter::new(conns, iter)
    }
}

impl<'a, Teams, Conns> ConnsInternal<'a, Teams, Conns>
where
    Conns: Deref<Target = ConnData>,
{
    /// Get all connections associated with a player
    pub fn associated_connections(&self, player: Entity) -> impl Iterator<Item = SocketId> + '_ {
        self.associated.get(player).map(|x| x.0).into_iter()
    }

    /// Get the connection associated
    pub fn player(&self, conn: SocketId) -> Result<Option<Entity>, NonexistantSocketError> {
        self.conns.player(conn)
    }

    /// Send a packet to the given connection.
    ///
    /// This method will take ownership of its arguments.
    /// If you don't want to clone the data every time,
    /// use [`send_to_ref()`][0] instead.
    ///
    /// [0]: #method.send_to_ref
    pub fn send_to<I>(&self, conn: SocketId, msg: I)
    where
        I: Into<ServerPacket>,
    {
        let msg = msg.into();
        self.send_to_ref(conn, &msg);
    }

    /// Send a packet to the given connection
    pub fn send_to_ref(&self, conn: SocketId, msg: &ServerPacket) {
        let segments = match encode_message(msg) {
            Ok(data) => data,
            Err(_) => {
                warn!("Failed to send message to socket {:?}", conn);
                return;
            }
        };

        for data in segments {
            if let Err(e) = self.conns.send_to(conn, data) {
                warn!("Unable to send to connection {:?}: {}", conn, e);
                break;
            }
        }
    }

    /// Send a packet to the primary connection for the player.
    pub fn send_to_player<I>(&self, player: Entity, msg: I)
    where
        I: Into<ServerPacket>,
    {
        self.send_to_player_ref(player, &msg.into());
    }

    /// Send a packet to the primary connection for the player.
    pub fn send_to_player_ref(&self, player: Entity, msg: &ServerPacket) {
        if let Some(conn) = self.associated.get(player) {
            self.send_to_ref(conn.0, msg);
        } else {
            warn!(
                "Tried to send message to player {:?} with no associated connection!",
                player
            );
        }
    }

    pub fn send_iter<'b: 'a>(&'b self) -> SendIter<impl Iterator<Item = Entity> + 'b, &'b Self> {
        let iter = (&self.entities, &self.is_player).join().map(|x| x.0);
        SendIter::new(&self, iter)
    }

    pub fn send_visible<'b: 'a>(
        &'b self,
        pos: Position,
    ) -> SendIter<impl Iterator<Item = Entity> + 'b, &'b Self> {
        let vals = self.grid.0.entity_collide(HitCircle {
            pos,
            rad: self.config.view_radius,
            layer: 0,
            ent: None,
        });

        SendIter::new(&self, vals.into_iter())
    }

    /// Send a packet to the primary connection for all players.
    pub fn send_to_all<I>(&self, msg: I)
    where
        I: Into<ServerPacket>,
    {
        self.send_to_all_ref(&msg.into());
    }

    /// Send a packet to the primary connection for all players.
    pub fn send_to_all_ref(&self, msg: &ServerPacket) {
        self.send_iter().send_all_ref(msg);
    }

    /// Send to all players except one.
    pub fn send_to_others<I>(&self, player: Entity, msg: I)
    where
        I: Into<ServerPacket>,
    {
        self.send_to_others_ref(player, &msg.into())
    }

    /// Send to all players except one.
    pub fn send_to_others_ref(&self, player: Entity, msg: &ServerPacket) {
        self.send_iter().except(player).send_all_ref(msg);
    }

    /// Send to all players that could see the given position.
    pub fn send_to_visible<I>(&self, pos: Position, msg: I)
    where
        I: Into<ServerPacket>,
    {
        self.send_visible(pos).send_all(msg);
    }

    /// Send to all players that could see the given position except one.
    pub fn send_to_visible_others<I>(&self, pos: Position, ent: Entity, msg: I)
    where
        I: Into<ServerPacket>,
    {
        self.send_visible(pos).except(ent).send_all(msg);
    }
}

impl<'a, Conns> ConnsInternal<'a, ReadStorage<'a, Team>, Conns>
where
    Conns: SystemData<'a> + Deref<Target = ConnData>,
{
    /// Send to all players on a team
    pub fn send_to_team<I>(&self, team: Team, msg: I)
    where
        I: Into<ServerPacket>,
    {
        self.send_iter().on_team(team).send_all(msg);
    }

    /// Send to all players on the given team that have
    /// the given position within their horizon.
    pub fn send_to_team_visible<I>(&self, pos: Position, team: Team, msg: I)
    where
        I: Into<ServerPacket>,
    {
        self.send_visible(pos).on_team(team).send_all(msg);
    }
}

impl<'a, Teams, Conns> ConnsInternal<'a, Teams, Conns>
where
    Teams: SystemData<'a>,
    Conns: SystemData<'a> + DerefMut<Target = ConnData>,
{
    pub fn associate(
        &mut self,
        id: SocketId,
        player: Entity,
    ) -> Result<(), NonexistantSocketError> {
        self.conns.associate(id, player)
    }

    pub fn close(&mut self, conn: SocketId) -> Result<(), NonexistantSocketError> {
        self.conns.close(conn)
    }
}

fn encode_message(msg: &ServerPacket) -> Result<impl Iterator<Item = RcBuf<u8>>, SerializeError> {
    ProtocolV5 {}
        .serialize_server(msg)
        .map(|iter| iter.map(RcBuf::new))
}

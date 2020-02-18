use airmash_protocol::*;
use airmash_protocol_v5::ProtocolV5;
use futures::{select, FutureExt, SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::{handshake::client::Request, Message};
use tokio_tungstenite::WebSocketStream;

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;

use std::f32::consts::PI;
use std::ops::{Add, Rem};
use std::time::{Duration, Instant};

use crate::config::BASE_DIR;
use crate::game::World;
use crate::ClientResult;

pub enum Timeout<T> {
    Value(T),
    Timeout(Instant),
}

pub struct Client<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub world: World,
    stream: WebSocketStream<S>,
}

impl Client<TcpStream> {
    pub async fn connect<R>(request: R) -> ClientResult<Self>
    where
        R: Into<Request<'static>> + Unpin,
    {
        let (stream, _) = tokio_tungstenite::connect_async(request).await?;

        Ok(Self {
            world: World::default(),
            stream,
        })
    }
}

impl<S> Client<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub async fn send_buf(&mut self, buf: Vec<u8>) -> ClientResult<()> {
        Ok(self.stream.send(Message::Binary(buf)).await?)
    }

    pub async fn send<'d, P>(&mut self, packet: P) -> ClientResult<()>
    where
        P: Into<ClientPacket<'d>>,
    {
        let packets = ProtocolV5 {}.serialize_client(&packet.into())?;

        for buf in packets {
            self.send_buf(buf).await?;
        }

        Ok(())
    }

    pub async fn next(&mut self) -> ClientResult<Option<ServerPacket<'static>>> {
        let buf = loop {
            let msg = match self.stream.next().await {
                Some(x) => x?,
                None => return Ok(None),
            };

            break match msg {
                Message::Binary(buf) => buf,
                Message::Ping(_) => continue,
                Message::Pong(_) => continue,
                Message::Text(txt) => txt.into_bytes(),
                Message::Close(_) => return Ok(None),
            };
        };

        let val = ProtocolV5 {}.deserialize_server(&buf)?;

        self.packet_update(&val).await?;

        Ok(Some(val))
    }

    async fn packet_update<'a>(&'a mut self, packet: &'a ServerPacket<'_>) -> ClientResult<()> {
        use self::ServerPacket::*;
        use airmash_protocol::client::Pong;

        self.world.handle_packet(dbg!(packet));

        match packet {
            Ping(p) => self.send(Pong { num: p.num }).await?,
            _ => (),
        };

        Ok(())
    }

    pub async fn quit(mut self) -> ClientResult<()> {
        self.stream.close(None).await?;

        Ok(())
    }
}

impl<S> Client<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    /// Just process packets indefinitely.
    pub async fn process_indefinitely(&mut self) -> ClientResult<()> {
        loop {
            let _ = self.next().await?;
        }
    }

    pub async fn next_timeout(
        &mut self,
        timeout: Duration,
    ) -> ClientResult<Timeout<Option<ServerPacket<'static>>>> {
        use tokio::time::{delay_until, Instant};

        let stop = Instant::now() + timeout;

        select! {
            _ = delay_until(stop).fuse() => Ok(Timeout::Timeout(std::time::Instant::now())),
            packet = self.next().fuse() => Ok(Timeout::Value(packet?))
        }
    }

    async fn _wait_for_pred<P>(
        &mut self,
        mut pred: P,
    ) -> ClientResult<Option<ServerPacket<'static>>>
    where
        P: FnMut(&ServerPacket<'static>) -> bool,
    {
        loop {
            let value = self.next().await?;
            match &value {
                Some(ref x) if pred(&x) => (),
                None => (),
                _ => continue,
            }

            return Ok(value);
        }
    }

    /// Wait until the predicate returns true or we time out.
    pub async fn wait_for_pred<P>(
        &mut self,
        timeout: Duration,
        pred: P,
    ) -> ClientResult<Timeout<Option<ServerPacket<'static>>>>
    where
        P: FnMut(&ServerPacket<'static>) -> bool,
    {
        use tokio::time::{delay_until, Instant};

        let stop = Instant::now() + timeout;

        select! {
            _ = delay_until(stop).fuse() => Ok(Timeout::Timeout(std::time::Instant::now())),
            packet = self._wait_for_pred(pred).fuse() => Ok(Timeout::Value(packet?))
        }
    }

    /// Press or release a key.
    ///
    /// This corresponds to the [`Key`] client packet.
    ///
    /// [`Key`]: protocol::client::Key
    pub async fn send_key(&mut self, key: KeyCode, state: bool) -> ClientResult<()> {
        use airmash_protocol::client::Key;

        let seq = self.world.key_seq;
        self.world.key_seq += 1;

        self.send(Key { key, seq, state }).await
    }

    /// Press a key.
    ///
    /// This corresponds to calling [`send_key`] with `true`.
    pub async fn press_key(&mut self, key: KeyCode) -> ClientResult<()> {
        self.send_key(key, true).await
    }

    /// Release a key.
    ///
    /// This corresponds to calling [`send_key`] with false.
    pub async fn release_key(&mut self, key: KeyCode) -> ClientResult<()> {
        self.send_key(key, false).await
    }

    /// Process events until the target time passes.
    pub async fn wait_until(&mut self, tgt: Instant) -> ClientResult<()> {
        use tokio::time::{delay_until, Instant};

        select! {
            _ = delay_until(Instant::from_std(tgt)).fuse() => Ok(()),
            x = self.process_indefinitely().fuse() => x
        }
    }

    /// Process events for the given duration.
    pub async fn wait(&mut self, dur: Duration) -> ClientResult<()> {
        self.wait_until(Instant::now() + dur).await
    }

    /// Turn the plane by a given rotation.
    ///
    /// This is a best effort implementation as it is
    /// impossible to turn exactly any given amount.
    /// This method may overshoot in cases where network
    /// ping changes significantly during the execution
    /// of the turn.
    pub async fn turn(&mut self, rot: Rotation) -> ClientResult<()> {
        let rotrate = crate::config::rotation_rate(self.world.get_me().plane);
        let time: Duration = (rot.abs() / rotrate).min(Time::new(100.0)).into();

        let key = if rot < 0.0.into() {
            KeyCode::Left
        } else {
            KeyCode::Right
        };

        if rot.inner().abs() < 0.05 {
            return Ok(());
        }

        self.press_key(key).await?;
        self.wait(time).await?;
        self.release_key(key).await?;

        Ok(())
    }

    /// Turn to a given angle.
    ///
    /// This is a best effort implementation as it is
    /// impossible to turn exactly any given amount.
    /// This method may overshoot in cases where network
    /// ping changes significantly during the execution
    /// of the turn.
    pub async fn turn_to(&mut self, tgt: Rotation) -> ClientResult<()> {
        /// Utility since rust doesn't provide fmod
        fn fmod<T>(a: T, b: T) -> T
        where
            T: Rem<Output = T> + Add<Output = T> + Copy,
        {
            (a % b + b) % b
        }

        // Determine the shortest turn angle
        // The basic idea comes from this SO answer
        // https://stackoverflow.com/questions/9505862/shortest-distance-between-two-degree-marks-on-a-circle
        let rot = self.world.get_me().rot;
        let pi = Rotation::new(PI);
        let pi2 = pi * 2.0;
        let mut dist = fmod(tgt - rot, pi2);

        if dist > pi {
            dist -= pi2;
        }

        self.turn(dist).await
    }

    /// Point the plane at a given point.
    ///
    /// This is a best effort implementation as it is
    /// impossible to turn exactly any given amount.
    /// This method may overshoot in cases where network
    /// ping changes significantly during the execution
    /// of the turn.
    pub async fn point_at(&mut self, pos: Position) -> ClientResult<()> {
        let rel = (pos - self.world.get_me().pos).normalized();
        let mut angle = Vector2::dot(rel, BASE_DIR).acos();

        if rel.x < 0.0.into() {
            angle = 2.0 * PI - angle;
        }

        self.turn_to(angle.into()).await
    }

    /// Say something in chat
    pub async fn chat(&mut self, text: &str) -> ClientResult<()> {
        self.send(client::Chat { text: text.into() }).await
    }

    /// Say something in a text bubble
    pub async fn team_chat(&mut self, text: &str) -> ClientResult<()> {
        self.send(client::TeamChat { text: text.into() }).await
    }

    /// Say something in a text bubble
    pub async fn say(&mut self, text: &str) -> ClientResult<()> {
        self.send(client::Say { text: text.into() }).await
    }

    /// Wait to receive a login packet. If the connection closes before
    /// receiving the packet then it will return `None`.
    pub async fn wait_for_login(&mut self) -> ClientResult<Option<server::Login<'static>>> {
        while let Some(x) = self.next().await? {
            if let ServerPacket::Login(p) = x {
                return Ok(Some(p));
            }
        }

        Ok(None)
    }

    /// Login to the server with the given name
    pub async fn login(&mut self, name: &str) -> ClientResult<Option<server::Login<'static>>> {
        use airmash_protocol::client::Login;

        let login = Login {
            flag: "JOLLY".into(),
            name: name.into(),
            horizon_x: 4000,
            horizon_y: 4000,
            protocol: 5,
            session: "".into(),
        };
        self.send(login).await?;

        self.wait_for_login().await
    }

    fn calc_angle(&mut self, pos: Position) -> f32 {
        let rel = (pos - self.world.get_me().pos).normalized();
        let mut angle = Vector2::dot(rel, BASE_DIR).acos();

        if rel.x < 0.0.into() {
            angle = 2.0 * PI - angle;
        }

        angle
    }

    pub async fn run_straight_at(&mut self, pos: Position) -> ClientResult<()> {
        self.point_at(pos).await?;
        self.press_key(KeyCode::Up).await?;
        let wait_duration = Duration::from_millis(self.world.ping as u64 * 2);
        self.wait(wait_duration).await?;

        loop {
            if let Timeout::Value(None) = self.next_timeout(Duration::from_millis(16)).await? {
                break;
            }

            let dist = (pos - self.world.get_me().pos).length();
            let angle = self.calc_angle(pos);

            if angle > 1.0 {
                if dist.inner() < 500.0 {
                    self.release_key(KeyCode::Up).await?;
                }

                self.point_at(pos).await?;

                if dist.inner() < 500.0 || !self.world.get_me().keystate.up {
                    self.press_key(KeyCode::Up).await?;
                }

                let wait_duration = Duration::from_millis(self.world.ping.into());
                self.wait(wait_duration).await?;
            }

            if dist.inner() < 100.0 {
                break;
            }
        }

        self.press_key(KeyCode::Down).await?;
        self.wait(Duration::from_millis(100)).await?;
        self.release_key(KeyCode::Down).await?;

        self.release_key(KeyCode::Up).await
    }

    /// Follow a player until the client shuts down.
    ///
    /// If you want to only follow a player for a certain amount of
    /// time, use select to only run this for a certiain amount of time.
    pub async fn follow(&mut self, player: u16) -> ClientResult<()> {
        let mut pos;
        let mut prev = Instant::now();
        self.press_key(KeyCode::Up).await?;

        loop {
            if let Timeout::Value(None) = self.next_timeout(Duration::from_millis(16)).await? {
                break;
            }

            if let Some(p) = self.world.players.get(&player) {
                pos = p.pos;

                let mypos = self.world.get_me().pos;
                if (pos - mypos).length() < 200.0.into() {
                    //break;
                }
            } else {
                break;
            }
            if Instant::now() - prev > Duration::from_millis(500) {
                self.press_key(KeyCode::Up).await?;
                prev = Instant::now();
            }

            self.point_at(pos).await?;
            let wait_duration =
                Duration::from_millis((self.world.ping * 2).min(1000).max(10) as u64);
            self.wait(wait_duration).await?;
        }

        self.release_key(KeyCode::Up).await
    }

    /// Send a server command. This is used for things such as respawning,
    /// admin commands, changing the flag, and more.
    pub async fn send_command(&mut self, cmd: &str, args: &str) -> ClientResult<()> {
        use airmash_protocol::client::Command;

        self.send(Command {
            com: cmd.into(),
            data: args.into(),
        })
        .await
    }
}

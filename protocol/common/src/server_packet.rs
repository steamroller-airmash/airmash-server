use crate::server::*;

/// All possible server packets.
///
/// This is an enum of all possible packet
/// message types.
///
/// Some packets do not contain any data
/// and thus do not have any data within
/// their enum variants.
///
/// The [`From`][0] trait has been implemented
/// for all the structs that correspond to the
/// variants of this enum. This means that instead
/// of directly constructing an instance of
/// `ServerPacket`, [`into()`][1] can be called
/// instead.
///
/// [0]: https://doc.rust-lang.org/std/convert/trait.From.html
/// [1]: https://doc.rust-lang.org/std/convert/trait.Into.html#tymethod.into
#[derive(Clone, Debug, From)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ServerPacket<'data> {
    Login(Login<'data>),
    Backup,
    Ping(Ping),
    PingResult(PingResult),
    Ack,
    Error(Error),
    CommandReply(CommandReply<'data>),
    PlayerNew(PlayerNew<'data>),
    PlayerLeave(PlayerLeave),
    PlayerUpdate(PlayerUpdate),
    PlayerFire(PlayerFire),
    PlayerRespawn(PlayerRespawn),
    PlayerFlag(PlayerFlag),
    PlayerHit(PlayerHit),
    PlayerKill(PlayerKill),
    PlayerUpgrade(PlayerUpgrade),
    PlayerType(PlayerType),
    PlayerPowerup(PlayerPowerup),
    PlayerLevel(PlayerLevel),
    PlayerReteam(PlayerReteam),
    GameFlag(GameFlag),
    GameSpectate(GameSpectate),
    GamePlayersAlive(GamePlayersAlive),
    GameFirewall(GameFirewall),
    EventRepel(EventRepel),
    EventBoost(EventBoost),
    EventBounce(EventBounce),
    EventStealth(EventStealth),
    EventLeaveHorizon(EventLeaveHorizon),
    MobUpdate(MobUpdate),
    MobUpdateStationary(MobUpdateStationary),
    MobDespawn(MobDespawn),
    MobDespawnCoords(MobDespawnCoords),
    ScoreUpdate(ScoreUpdate),
    ScoreBoard(ScoreBoard),
    ScoreDetailedFFA(ScoreDetailedFFA),
    ScoreDetailedCTF(ScoreDetailedCTF),
    ScoreDetailedBTR(ScoreDetailedBTR),
    ChatTeam(ChatTeam<'data>),
    ChatPublic(ChatPublic<'data>),
    ChatSay(ChatSay<'data>),
    ChatWhisper(ChatWhisper<'data>),
    ChatVoteMutePassed(ChatVoteMutePassed),
    ChatVoteMuted,
    ServerMessage(ServerMessage<'data>),
    ServerCustom(ServerCustom<'data>),
}

macro_rules! impl_from_empty {
    ($type:tt) => {
        impl_from_empty_inner!(ServerPacket, $type);
    };
}

impl_from_empty!(Backup);
impl_from_empty!(Ack);
impl_from_empty!(ChatVoteMuted);

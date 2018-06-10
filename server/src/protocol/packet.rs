
use protocol::server::*;
use protocol::client::*;
use protocol::serde_am::*;

use protocol::client::Login as ClientLogin;
use protocol::server::Login as ServerLogin;

/// All possible server packets.
/// 
/// This is an enum of all possible packet 
/// message types. It can be serialized 
/// and deserialized from byte buffers
/// using [`from_bytes`](fn.from_bytes.html)
/// and [`to_bytes`](fn.to_bytes.html).
/// 
/// Some packets do not contain any data
/// and thus do not have any data within
/// their enum variants.
#[derive(Clone, Debug)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub enum ServerPacket {
    Login(ServerLogin),
    Backup,
    Ping(Ping),
    PingResult(PingResult),
    Ack,
    Error(Error),
    CommandReply(CommandReply),
    PlayerNew(PlayerNew),
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
    GameFireWall(GameFirewall),
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
    ChatTeam(ChatTeam),
    ChatPublic(ChatPublic),
    ChatSay(ChatSay),
    ChatWhisper(ChatWhisper),
    ChatVoteMutePassed(ChatVotemutePassed),
    ChatVoteMuted,
    ServerMessage(ServerMessage),
    ServerCustom(ServerCustom),
}

impl ServerPacket {
    /// Gets the id of the packet associated
    /// with the current packet type.
    pub fn variant_id(&self) -> u8 {
        use protocol::codes::server::*;

        match self {
            &ServerPacket::Login(_) => LOGIN,
            &ServerPacket::Backup => BACKUP,
            &ServerPacket::Ping(_) => PING,
            &ServerPacket::PingResult(_) => PING_RESULT,
            &ServerPacket::Ack => ACK,
            &ServerPacket::Error(_) => ERROR,
            &ServerPacket::CommandReply(_) => COMMAND_REPLY,
            &ServerPacket::PlayerNew(_) => PLAYER_NEW,
            &ServerPacket::PlayerLeave(_) => PLAYER_LEAVE,
            &ServerPacket::PlayerUpdate(_) => PLAYER_UPDATE,
            &ServerPacket::PlayerFire(_) => PLAYER_FIRE,
            &ServerPacket::PlayerHit(_) => PLAYER_HIT,
            &ServerPacket::PlayerRespawn(_) => PLAYER_RESPAWN,
            &ServerPacket::PlayerFlag(_) => PLAYER_FLAG,
            &ServerPacket::PlayerKill(_) => PLAYER_KILL,
            &ServerPacket::PlayerUpgrade(_) => PLAYER_UPGRADE,
            &ServerPacket::PlayerType(_) => PLAYER_TYPE,
            &ServerPacket::PlayerPowerup(_) => PLAYER_POWERUP,
            &ServerPacket::PlayerLevel(_) => PLAYER_LEVEL,
            &ServerPacket::PlayerReteam(_) => PLAYER_RETEAM,
            &ServerPacket::GameFlag(_) => GAME_FLAG,
            &ServerPacket::GameSpectate(_) => GAME_SPECTATE,
            &ServerPacket::GamePlayersAlive(_) => GAME_PLAYERSALIVE,
            &ServerPacket::GameFireWall(_) => GAME_FIREWALL,
            &ServerPacket::EventRepel(_) => EVENT_REPEL,
            &ServerPacket::EventBoost(_) => EVENT_BOOST,
            &ServerPacket::EventBounce(_) => EVENT_BOUNCE,
            &ServerPacket::EventStealth(_) => EVENT_STEALTH,
            &ServerPacket::EventLeaveHorizon(_) => EVENT_LEAVEHORIZON,
            &ServerPacket::MobUpdate(_) => MOB_UPDATE,
            &ServerPacket::MobUpdateStationary(_) => MOB_UPDATE_STATIONARY,
            &ServerPacket::MobDespawn(_) => MOB_DESPAWN,
            &ServerPacket::MobDespawnCoords(_) => MOB_DESPAWN_COORDS,
            &ServerPacket::ChatPublic(_) => CHAT_PUBLIC,
            &ServerPacket::ChatTeam(_) => CHAT_TEAM,
            &ServerPacket::ChatSay(_) => CHAT_SAY,
            &ServerPacket::ChatWhisper(_) => CHAT_WHISPER,
            &ServerPacket::ChatVoteMutePassed(_) => CHAT_VOTEMUTEPASSED,
            &ServerPacket::ChatVoteMuted => CHAT_VOTEMUTED,
            &ServerPacket::ScoreUpdate(_) => SCORE_UPDATE,
            &ServerPacket::ScoreBoard(_) => SCORE_BOARD,
            &ServerPacket::ScoreDetailedFFA(_) => SCORE_DETAILED_FFA,
            &ServerPacket::ScoreDetailedCTF(_) => SCORE_DETAILED_CTF,
            &ServerPacket::ScoreDetailedBTR(_) => SCORE_DETAILED_BTR,
            &ServerPacket::ServerMessage(_) => SERVER_MESSAGE,
            &ServerPacket::ServerCustom(_) => SERVER_CUSTOM
        }
    }
}


/// All possible client packets.
/// 
/// This contains all valid packets that
/// the client can send to the server
/// (in the current version of the airmash
/// protocol). It can be serialized and 
/// deserialized to/from byte buffers
/// using [`to_bytes`](fn.to_bytes.html)
/// and [`from_bytes`](fn.from_bytes.html).
/// 
/// Some packets don't contain any data, these
/// packets do not have an associated struct
/// and as such are just empty variants within
/// this enum.
#[derive(Clone, Debug)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub enum ClientPacket {
    Login(ClientLogin),
    Backup(Backup),
    Horizon(Horizon),
    Ack,
    Pong(Pong),
    Key(Key),
    Command(Command),
    ScoreDetailed,
    Chat(Chat),
    TeamChat(TeamChat),
    Whisper(Whisper),
    Say(Say),
    VoteMute(VoteMute),
    LocalPing(LocalPing),
}




use protocol::client::ClientPacket;
use protocol::server::ServerPacket;

use protocol::serde_am::*;
use protocol::error;

fn ser_w_code<T>(code: u8, v: &T, ser: &mut Serializer) -> Result<(), SerError>
where
    T: Serialize
{
    code.serialize(ser)?;
    v.serialize(ser)
}

impl Serialize for ClientPacket {
    fn serialize(&self, ser: &mut Serializer) -> Result<(), SerError> {
        use ::protocol::codes::client::*;

        match self {
            &ClientPacket::Login(ref p) => ser_w_code(LOGIN, p, ser),
            &ClientPacket::Backup(ref p) => ser_w_code(BACKUP, p, ser),
            &ClientPacket::Horizon(ref p) => ser_w_code(HORIZON, p, ser),
            &ClientPacket::Ack => ACK.serialize(ser),
            &ClientPacket::Pong(ref p) => ser_w_code(PONG, p, ser),
            &ClientPacket::Key(ref p) => ser_w_code(KEY, p, ser),
            &ClientPacket::Command(ref p) => ser_w_code(COMMAND, p, ser),
            &ClientPacket::ScoreDetailed => SCORE_DETAILED.serialize(ser),
            &ClientPacket::Chat(ref p) => ser_w_code(CHAT, p, ser),
            &ClientPacket::TeamChat(ref p) => ser_w_code(TEAMCHAT, p, ser),
            &ClientPacket::Whisper(ref p) => ser_w_code(WHISPER, p, ser),
            &ClientPacket::Say(ref p) => ser_w_code(SAY, p, ser),
            &ClientPacket::VoteMute(ref p) => ser_w_code(VOTEMUTE, p, ser),
            &ClientPacket::LocalPing(ref p) => ser_w_code(LOCALPING, p, ser)
        }
    }
}
impl<'de> Deserialize<'de> for ClientPacket {
    fn deserialize(de: &mut Deserializer<'de>) -> Result<ClientPacket, DeError> {
        use protocol::codes::client::*;
        use protocol::client::*;

        Ok(match de.deserialize_u8()? {
            LOGIN => ClientPacket::Login(Login::deserialize(de)?),
            BACKUP => ClientPacket::Backup(Backup::deserialize(de)?),
            HORIZON => ClientPacket::Horizon(Horizon::deserialize(de)?),
            ACK => ClientPacket::Ack,
            PONG => ClientPacket::Pong(Pong::deserialize(de)?),
            KEY => ClientPacket::Key(Key::deserialize(de)?),
            COMMAND => ClientPacket::Command(Command::deserialize(de)?),
            SCORE_DETAILED => ClientPacket::ScoreDetailed,
            CHAT => ClientPacket::Chat(Chat::deserialize(de)?),
            WHISPER => ClientPacket::Whisper(Whisper::deserialize(de)?),
            SAY => ClientPacket::Say(Say::deserialize(de)?),
            TEAMCHAT => ClientPacket::TeamChat(TeamChat::deserialize(de)?),
            VOTEMUTE => ClientPacket::VoteMute(VoteMute::deserialize(de)?),
            LOCALPING => ClientPacket::LocalPing(LocalPing::deserialize(de)?),
            _ => return Err(error::DeError::InvalidPacketType)
        })
    }
}

impl Serialize for ServerPacket {
    fn serialize(&self, ser: &mut Serializer) -> Result<(), SerError> {
        use protocol::codes::server::*;

        match self {
            &ServerPacket::Login(ref p) => ser_w_code(LOGIN, p, ser),
            &ServerPacket::Backup => BACKUP.serialize(ser),
            &ServerPacket::Ping(ref p) => ser_w_code(PING, p, ser),
            &ServerPacket::PingResult(ref p) => ser_w_code(PING_RESULT, p, ser),
            &ServerPacket::Ack => ACK.serialize(ser),
            &ServerPacket::CommandReply(ref p) => ser_w_code(COMMAND_REPLY, p, ser),
            &ServerPacket::PlayerNew(ref p) => ser_w_code(PLAYER_NEW, p, ser),
            &ServerPacket::PlayerLeave(ref p) => ser_w_code(PLAYER_LEAVE, p, ser),
            &ServerPacket::PlayerUpdate(ref p) => ser_w_code(PLAYER_UPDATE, p, ser),
            &ServerPacket::PlayerFire(ref p) => ser_w_code(PLAYER_FIRE, p, ser),
            &ServerPacket::PlayerRespawn(ref p) => ser_w_code(PLAYER_RESPAWN, p, ser),
            &ServerPacket::PlayerFlag(ref p) => ser_w_code(PLAYER_FLAG, p, ser),
            &ServerPacket::PlayerHit(ref p) => ser_w_code(PLAYER_HIT, p, ser),
            &ServerPacket::PlayerKill(ref p) => ser_w_code(PLAYER_KILL, p, ser),
            &ServerPacket::PlayerType(ref p) => ser_w_code(PLAYER_TYPE, p, ser),
            &ServerPacket::PlayerLevel(ref p) => ser_w_code(PLAYER_LEVEL, p, ser),
            &ServerPacket::PlayerReteam(ref p) => ser_w_code(PLAYER_RETEAM, p, ser),
            &ServerPacket::GameFlag(ref p) => ser_w_code(GAME_FLAG, p, ser),
            &ServerPacket::GameSpectate(ref p) => ser_w_code(GAME_SPECTATE, p, ser),
            &ServerPacket::GamePlayersAlive(ref p) => ser_w_code(GAME_PLAYERSALIVE, p, ser),
            &ServerPacket::GameFireWall(ref p) => ser_w_code(GAME_FIREWALL, p, ser),
            &ServerPacket::EventRepel(ref p) => ser_w_code(EVENT_REPEL, p, ser),
            &ServerPacket::EventBoost(ref p) => ser_w_code(EVENT_BOOST, p, ser),
            &ServerPacket::EventBounce(ref p) => ser_w_code(EVENT_BOUNCE, p, ser),
            &ServerPacket::EventStealth(ref p) => ser_w_code(EVENT_STEALTH, p, ser),
            &ServerPacket::EventLeaveHorizon(ref p) => ser_w_code(EVENT_LEAVEHORIZON, p, ser),
            &ServerPacket::MobUpdate(ref p) => ser_w_code(MOB_UPDATE, p, ser),
            &ServerPacket::MobUpdateStationary(ref p) => ser_w_code(MOB_UPDATE_STATIONARY, p, ser),
            &ServerPacket::MobDespawn(ref p) => ser_w_code(MOB_DESPAWN, p, ser),
            &ServerPacket::MobDespawnCoords(ref p) => ser_w_code(MOB_DESPAWN_COORDS, p, ser),
            &ServerPacket::ScoreUpdate(ref p) => ser_w_code(SCORE_UPDATE, p, ser),
            &ServerPacket::ScoreDetailedFFA(ref p) => ser_w_code(SCORE_DETAILED_FFA, p, ser),
            &ServerPacket::ScoreDetailedCTF(ref p) => ser_w_code(SCORE_DETAILED_CTF, p, ser),
            &ServerPacket::ScoreDetailedBTR(ref p) => ser_w_code(SCORE_DETAILED_BTR, p, ser),
            &ServerPacket::ChatTeam(ref p) => ser_w_code(CHAT_TEAM, p, ser),
            &ServerPacket::ChatPublic(ref p) => ser_w_code(CHAT_PUBLIC, p, ser),
            &ServerPacket::ChatSay(ref p) => ser_w_code(CHAT_SAY, p, ser),
            &ServerPacket::ChatWhisper(ref p) => ser_w_code(CHAT_WHISPER, p, ser),
            &ServerPacket::ChatVoteMutePassed(ref p) => ser_w_code(CHAT_VOTEMUTEPASSED, p, ser),
            &ServerPacket::ChatVoteMuted => CHAT_VOTEMUTED.serialize(ser),
            &ServerPacket::ServerMessage(ref p) => ser_w_code(SERVER_MESSAGE, p, ser),
            &ServerPacket::ServerCustom(ref p) => ser_w_code(SERVER_CUSTOM, p, ser),
            &ServerPacket::ScoreBoard(ref p) => ser_w_code(SCORE_BOARD, p, ser),
            &ServerPacket::PlayerUpgrade(ref p) => ser_w_code(PLAYER_UPGRADE, p, ser),
            &ServerPacket::PlayerPowerup(ref p) => ser_w_code(PLAYER_POWERUP, p, ser),
            &ServerPacket::Error(ref p) => ser_w_code(ERROR, p, ser)
        }
    }
}
impl<'de> Deserialize<'de> for ServerPacket {
    fn deserialize(de: &mut Deserializer<'de>) -> Result<ServerPacket, DeError> {
        use protocol::codes::server::*;
        use protocol::server::*;
        use protocol::server;

        Ok(match de.deserialize_u8()? {
            LOGIN => ServerPacket::Login(Login::deserialize(de)?),
            BACKUP => ServerPacket::Backup,
            PING => ServerPacket::Ping(Ping::deserialize(de)?),
            PING_RESULT => ServerPacket::PingResult(PingResult::deserialize(de)?),
            ACK => ServerPacket::Ack,
            ERROR => ServerPacket::Error(server::Error::deserialize(de)?),
            COMMAND_REPLY => ServerPacket::CommandReply(CommandReply::deserialize(de)?),
            PLAYER_NEW => ServerPacket::PlayerNew(PlayerNew::deserialize(de)?),
            PLAYER_LEAVE => ServerPacket::PlayerLeave(PlayerLeave::deserialize(de)?),
            PLAYER_UPDATE => ServerPacket::PlayerUpdate(PlayerUpdate::deserialize(de)?),
            PLAYER_FIRE => ServerPacket::PlayerFire(PlayerFire::deserialize(de)?),
            PLAYER_HIT => ServerPacket::PlayerHit(PlayerHit::deserialize(de)?),
            PLAYER_RESPAWN => ServerPacket::PlayerRespawn(PlayerRespawn::deserialize(de)?),
            PLAYER_FLAG => ServerPacket::PlayerFlag(PlayerFlag::deserialize(de)?),
            PLAYER_KILL => ServerPacket::PlayerKill(PlayerKill::deserialize(de)?),
            PLAYER_UPGRADE => ServerPacket::PlayerUpgrade(PlayerUpgrade::deserialize(de)?),
            PLAYER_TYPE => ServerPacket::PlayerType(PlayerType::deserialize(de)?),
            PLAYER_POWERUP => ServerPacket::PlayerPowerup(PlayerPowerup::deserialize(de)?),
            PLAYER_LEVEL => ServerPacket::PlayerLevel(PlayerLevel::deserialize(de)?),
            PLAYER_RETEAM => ServerPacket::PlayerReteam(PlayerReteam::deserialize(de)?),
            GAME_FLAG => ServerPacket::GameFlag(GameFlag::deserialize(de)?),
            GAME_SPECTATE => ServerPacket::GameSpectate(GameSpectate::deserialize(de)?),
            GAME_PLAYERSALIVE => ServerPacket::GamePlayersAlive(GamePlayersAlive::deserialize(de)?),
            GAME_FIREWALL => ServerPacket::GameFireWall(GameFirewall::deserialize(de)?),
            EVENT_REPEL => ServerPacket::EventRepel(EventRepel::deserialize(de)?),
            EVENT_BOOST => ServerPacket::EventBoost(EventBoost::deserialize(de)?),
            EVENT_BOUNCE => ServerPacket::EventBounce(EventBounce::deserialize(de)?),
            EVENT_STEALTH => ServerPacket::EventStealth(EventStealth::deserialize(de)?),
            EVENT_LEAVEHORIZON => ServerPacket::EventLeaveHorizon(EventLeaveHorizon::deserialize(de)?),
            MOB_UPDATE => ServerPacket::MobUpdate(MobUpdate::deserialize(de)?),
            MOB_UPDATE_STATIONARY => ServerPacket::MobUpdateStationary(MobUpdateStationary::deserialize(de)?),
            MOB_DESPAWN => ServerPacket::MobDespawn(MobDespawn::deserialize(de)?),
            MOB_DESPAWN_COORDS => ServerPacket::MobDespawnCoords(MobDespawnCoords::deserialize(de)?),
            CHAT_PUBLIC => ServerPacket::ChatPublic(ChatPublic::deserialize(de)?),
            CHAT_TEAM => ServerPacket::ChatTeam(ChatTeam::deserialize(de)?),
            CHAT_SAY => ServerPacket::ChatSay(ChatSay::deserialize(de)?),
            CHAT_WHISPER => ServerPacket::ChatWhisper(ChatWhisper::deserialize(de)?),
            CHAT_VOTEMUTEPASSED => ServerPacket::ChatVoteMutePassed(ChatVotemutePassed::deserialize(de)?),
            CHAT_VOTEMUTED => ServerPacket::ChatVoteMuted,
            SCORE_UPDATE => ServerPacket::ScoreUpdate(ScoreUpdate::deserialize(de)?),
            SCORE_BOARD => ServerPacket::ScoreBoard(ScoreBoard::deserialize(de)?),
            SCORE_DETAILED_FFA => ServerPacket::ScoreDetailedFFA(ScoreDetailedFFA::deserialize(de)?),
            SCORE_DETAILED_CTF => ServerPacket::ScoreDetailedCTF(ScoreDetailedCTF::deserialize(de)?),
            SCORE_DETAILED_BTR => ServerPacket::ScoreDetailedBTR(ScoreDetailedBTR::deserialize(de)?),
            SERVER_MESSAGE => ServerPacket::ServerMessage(ServerMessage::deserialize(de)?),
            SERVER_CUSTOM => ServerPacket::ServerCustom(ServerCustom::deserialize(de)?),
            _ => return Err(error::DeError::InvalidPacketType)
        })
    }
}

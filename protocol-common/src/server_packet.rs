use server::*;

/// All possible server packets.
///
/// This is an enum of all possible packet
/// message types.
///
/// Some packets do not contain any data
/// and thus do not have any data within
/// their enum variants.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerPacket {
	Login(Login),
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
	ChatTeam(ChatTeam),
	ChatPublic(ChatPublic),
	ChatSay(ChatSay),
	ChatWhisper(ChatWhisper),
	ChatVoteMutePassed(ChatVoteMutePassed),
	ChatVoteMuted,
	ServerMessage(ServerMessage),
	ServerCustom(ServerCustom),
}

macro_rules! impl_from {
	($type:tt) => {
		impl From<$type> for ServerPacket {
			fn from(v: $type) -> Self {
				ServerPacket::$type(v)
			}
		}
	};
}

impl_from!(Login);
impl_from!(Ping);
impl_from!(PingResult);
impl_from!(Error);
impl_from!(CommandReply);
impl_from!(PlayerNew);
impl_from!(PlayerLeave);
impl_from!(PlayerUpdate);
impl_from!(PlayerFire);
impl_from!(PlayerRespawn);
impl_from!(PlayerFlag);
impl_from!(PlayerHit);
impl_from!(PlayerKill);
impl_from!(PlayerUpgrade);
impl_from!(PlayerType);
impl_from!(PlayerPowerup);
impl_from!(PlayerLevel);
impl_from!(PlayerReteam);
impl_from!(GameFlag);
impl_from!(GameSpectate);
impl_from!(GamePlayersAlive);
impl_from!(GameFirewall);
impl_from!(EventRepel);
impl_from!(EventBoost);
impl_from!(EventBounce);
impl_from!(EventStealth);
impl_from!(EventLeaveHorizon);
impl_from!(MobUpdate);
impl_from!(MobUpdateStationary);
impl_from!(MobDespawn);
impl_from!(MobDespawnCoords);
impl_from!(ScoreUpdate);
impl_from!(ScoreBoard);
impl_from!(ScoreDetailedFFA);
impl_from!(ScoreDetailedCTF);
impl_from!(ScoreDetailedBTR);
impl_from!(ChatTeam);
impl_from!(ChatPublic);
impl_from!(ChatSay);
impl_from!(ChatWhisper);
impl_from!(ChatVoteMutePassed);
impl_from!(ServerMessage);
impl_from!(ServerCustom);

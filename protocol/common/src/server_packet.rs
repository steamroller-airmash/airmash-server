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
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

macro_rules! impl_from_newtype {
	($type:tt) => {
		impl_from_newtype_inner!(ServerPacket, $type);
	};
}

macro_rules! impl_from_empty {
	($type:tt) => {
		impl_from_empty_inner!(ServerPacket, $type);
	};
}

impl_from_newtype!(Login);
impl_from_newtype!(Ping);
impl_from_newtype!(PingResult);
impl_from_newtype!(Error);
impl_from_newtype!(CommandReply);
impl_from_newtype!(PlayerNew);
impl_from_newtype!(PlayerLeave);
impl_from_newtype!(PlayerUpdate);
impl_from_newtype!(PlayerFire);
impl_from_newtype!(PlayerRespawn);
impl_from_newtype!(PlayerFlag);
impl_from_newtype!(PlayerHit);
impl_from_newtype!(PlayerKill);
impl_from_newtype!(PlayerUpgrade);
impl_from_newtype!(PlayerType);
impl_from_newtype!(PlayerPowerup);
impl_from_newtype!(PlayerLevel);
impl_from_newtype!(PlayerReteam);
impl_from_newtype!(GameFlag);
impl_from_newtype!(GameSpectate);
impl_from_newtype!(GamePlayersAlive);
impl_from_newtype!(GameFirewall);
impl_from_newtype!(EventRepel);
impl_from_newtype!(EventBoost);
impl_from_newtype!(EventBounce);
impl_from_newtype!(EventStealth);
impl_from_newtype!(EventLeaveHorizon);
impl_from_newtype!(MobUpdate);
impl_from_newtype!(MobUpdateStationary);
impl_from_newtype!(MobDespawn);
impl_from_newtype!(MobDespawnCoords);
impl_from_newtype!(ScoreUpdate);
impl_from_newtype!(ScoreBoard);
impl_from_newtype!(ScoreDetailedFFA);
impl_from_newtype!(ScoreDetailedCTF);
impl_from_newtype!(ScoreDetailedBTR);
impl_from_newtype!(ChatTeam);
impl_from_newtype!(ChatPublic);
impl_from_newtype!(ChatSay);
impl_from_newtype!(ChatWhisper);
impl_from_newtype!(ChatVoteMutePassed);
impl_from_newtype!(ServerMessage);
impl_from_newtype!(ServerCustom);

impl_from_empty!(Backup);
impl_from_empty!(Ack);
impl_from_empty!(ChatVoteMuted);

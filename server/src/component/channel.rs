use shrev::*;

use component::event::*;
use types::event::{ConnectionClose, ConnectionOpen};

// Connection Events
pub type OnOpen = EventChannel<ConnectionOpen>;
pub type OnClose = EventChannel<ConnectionClose>;

// Timer Event
pub type OnTimerEvent = EventChannel<TimerEvent>;

// Packet Received Events
pub type OnBinary = EventChannel<BinaryEvent>;
pub type OnLogin = EventChannel<LoginEvent>;
pub type OnBackup = EventChannel<BackupEvent>;
pub type OnCommand = EventChannel<CommandEvent>;
pub type OnHorizon = EventChannel<HorizonEvent>;
pub type OnKey = EventChannel<KeyEvent>;
pub type OnPong = EventChannel<PongEvent>;
pub type OnChat = EventChannel<ChatEvent>;
pub type OnSay = EventChannel<SayEvent>;
pub type OnTeamChat = EventChannel<TeamChatEvent>;
pub type OnWhisper = EventChannel<WhisperEvent>;
pub type OnVotemute = EventChannel<VotemuteEvent>;
pub type OnLocalPing = EventChannel<LocalPingEvent>;
pub type OnScoreDetailed = EventChannel<ScoreDetailedEvent>;
pub type OnAck = EventChannel<AckEvent>;
pub type OnAnyChatEvent = EventChannel<AnyChatEvent>;

// In-game events
pub type OnPlayerJoin = EventChannel<PlayerJoin>;
pub type OnPlayerLeave = EventChannel<PlayerLeave>;
pub type OnPlayerKilled = EventChannel<PlayerKilled>;
pub type OnPlayerRespawn = EventChannel<PlayerRespawn>;
pub type OnPlayerSpectate = EventChannel<PlayerSpectate>;
pub type OnPlayerStealth = EventChannel<PlayerStealth>;
pub type OnMissileFire = EventChannel<MissileFire>;
pub type OnPlayerRepel = EventChannel<PlayerRepel>;
pub type OnPlayerMuted = EventChannel<PlayerMute>;
pub type OnPlayerThrottled = EventChannel<PlayerThrottle>;
pub type OnPlayerHit = EventChannel<PlayerHit>;
pub type OnPowerupExpired = EventChannel<PowerupExpired>;
pub type OnPlayerPowerup = EventChannel<PlayerPowerup>;
pub type OnPlayerDespawn = EventChannel<PlayerDespawn>;

// Upgrade Events
pub type OnUpgradeSpawn = EventChannel<UpgradeSpawnEvent>;
pub type OnUpgradePickup = EventChannel<UpgradePickupEvent>;
pub type OnUpgradeDespawn = EventChannel<UpgradeDespawnEvent>;

// Collision events
pub type OnPlayerTerrainCollision = EventChannel<PlayerTerrainCollision>;
pub type OnPlayerMissileCollision = EventChannel<PlayerMissileCollision>;
pub type OnPlayerPowerupCollision = EventChannel<PlayerPowerupCollision>;
pub type OnMissileTerrainCollision = EventChannel<MissileTerrainCollision>;
pub type OnPlayerUpgradeCollision = EventChannel<PlayerUpgradeCollision>;

// Readers
pub type OnOpenReader = ReaderId<ConnectionOpen>;
pub type OnCloseReader = ReaderId<ConnectionClose>;

pub type OnTimerEventReader = ReaderId<TimerEvent>;

pub type OnBinaryReader = ReaderId<BinaryEvent>;
pub type OnLoginReader = ReaderId<LoginEvent>;
pub type OnBackupReader = ReaderId<BackupEvent>;
pub type OnCommandReader = ReaderId<CommandEvent>;
pub type OnHorizonReader = ReaderId<HorizonEvent>;
pub type OnKeyReader = ReaderId<KeyEvent>;
pub type OnPongReader = ReaderId<PongEvent>;
pub type OnChatReader = ReaderId<ChatEvent>;
pub type OnSayReader = ReaderId<SayEvent>;
pub type OnTeamChatReader = ReaderId<TeamChatEvent>;
pub type OnWhisperReader = ReaderId<WhisperEvent>;
pub type OnVotemuteReader = ReaderId<VotemuteEvent>;
pub type OnLocalPingReader = ReaderId<LocalPingEvent>;
pub type OnScoreDetailedReader = ReaderId<ScoreDetailedEvent>;
pub type OnAckReader = ReaderId<AckEvent>;
pub type OnChatEventReader = ReaderId<AnyChatEvent>;

// In-game events
pub type OnPlayerJoinReader = ReaderId<PlayerJoin>;
pub type OnPlayerLeaveReader = ReaderId<PlayerLeave>;
pub type OnPlayerKilledReader = ReaderId<PlayerKilled>;
pub type OnPlayerRespawnReader = ReaderId<PlayerRespawn>;
pub type OnPlayerSpectateReader = ReaderId<PlayerSpectate>;
pub type OnPlayerStealthReader = ReaderId<PlayerStealth>;
pub type OnMissileFireReader = ReaderId<MissileFire>;
pub type OnPlayerRepelReader = ReaderId<PlayerRepel>;
pub type OnPlayerMutedReader = ReaderId<PlayerMute>;
pub type OnPlayerThrottledReader = ReaderId<PlayerThrottle>;
pub type OnPlayerHitReader = ReaderId<PlayerHit>;
pub type OnPowerupExpiredReader = ReaderId<PowerupExpired>;
pub type OnPlayerPowerupReader = ReaderId<PlayerPowerup>;
pub type OnPlayerDespawnReader = ReaderId<PlayerDespawn>;

// Upgrade Events
pub type OnUpgradePickupReader = ReaderId<UpgradePickupEvent>;

// Collision events
pub type OnPlayerMissileCollisionReader = ReaderId<PlayerMissileCollision>;
pub type OnPlayerTerrainCollisionReader = ReaderId<PlayerTerrainCollision>;
pub type OnPlayerPowerupCollisionReader = ReaderId<PlayerPowerupCollision>;
pub type OnMissileTerrainCollisionReader = ReaderId<MissileTerrainCollision>;
pub type OnPlayerUpgradeCollisionReader = ReaderId<PlayerUpgradeCollision>;

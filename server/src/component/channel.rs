use shrev::*;

use protocol::client::*;
use types::ConnectionId;
use component::event::*;
use types::event::{ConnectionOpen, ConnectionClose, Message};

// Connection Events
pub type OnOpen = EventChannel<ConnectionOpen>;
pub type OnClose = EventChannel<ConnectionClose>;

// Timer Event
pub type OnTimerEvent = EventChannel<TimerEvent>;

// Packet Received Events
pub type OnBinary = EventChannel<Message>;
pub type OnLogin = EventChannel<(ConnectionId, Login)>;
pub type OnBackup = EventChannel<(ConnectionId, Backup)>;
pub type OnCommand = EventChannel<(ConnectionId, Command)>;
pub type OnHorizon = EventChannel<(ConnectionId, Horizon)>;
pub type OnKey = EventChannel<(ConnectionId, Key)>;
pub type OnPong = EventChannel<(ConnectionId, Pong)>;
pub type OnChat = EventChannel<(ConnectionId, Chat)>;
pub type OnSay = EventChannel<(ConnectionId, Say)>;
pub type OnTeamChat = EventChannel<(ConnectionId, TeamChat)>;
pub type OnWhisper = EventChannel<(ConnectionId, Whisper)>;
pub type OnVotemute = EventChannel<(ConnectionId, VoteMute)>;
pub type OnLocalPing = EventChannel<(ConnectionId, LocalPing)>;
pub type OnScoreDetailed = EventChannel<ScoreDetailedEvent>;
pub type OnAck = EventChannel<AckEvent>;

// In-game events
pub type OnPlayerJoin = EventChannel<PlayerJoin>;
pub type OnPlayerLeave = EventChannel<PlayerLeave>;
pub type OnPlayerKilled = EventChannel<PlayerKilled>;

// Collision events
pub type OnPlayerTerrainCollision = EventChannel<PlayerTerrainCollision>;
pub type OnPlayerMissileCollision = EventChannel<PlayerMissileCollision>;
pub type OnPlayerPowerupCollision = EventChannel<PlayerPowerupCollision>;
pub type OnMissileTerrainCollision = EventChannel<MissileTerrainCollision>;

// Readers
pub type OnOpenReader = ReaderId<ConnectionOpen>;
pub type OnCloseReader = ReaderId<ConnectionClose>;

pub type OnTimerEventReader = ReaderId<TimerEvent>;

pub type OnBinaryReader = ReaderId<Message>;
pub type OnLoginReader = ReaderId<(ConnectionId, Login)>;
pub type OnBackupReader = ReaderId<(ConnectionId, Backup)>;
pub type OnCommandReader = ReaderId<(ConnectionId, Command)>;
pub type OnHorizonReader = ReaderId<(ConnectionId, Horizon)>;
pub type OnKeyReader = ReaderId<(ConnectionId, Key)>;
pub type OnPongReader = ReaderId<(ConnectionId, Pong)>;
pub type OnChatReader = ReaderId<(ConnectionId, Chat)>;
pub type OnSayReader = ReaderId<(ConnectionId, Say)>;
pub type OnTeamChatReader = ReaderId<(ConnectionId, TeamChat)>;
pub type OnWhisperReader = ReaderId<(ConnectionId, Whisper)>;
pub type OnVotemuteReader = ReaderId<(ConnectionId, VoteMute)>;
pub type OnLocalPingReader = ReaderId<(ConnectionId, LocalPing)>;
pub type OnScoreDetailedReader = ReaderId<ScoreDetailedEvent>;
pub type OnAckReader = ReaderId<AckEvent>;

// In-game events
pub type OnPlayerJoinReader = ReaderId<PlayerJoin>;
pub type OnPlayerLeaveReader = ReaderId<PlayerLeave>;
pub type OnPlayerKilledReader = ReaderId<PlayerKilled>;

// Collision events
pub type OnPlayerMissileCollisionReader = ReaderId<PlayerMissileCollision>;
pub type OnPlayerTerrainCollisionReader = ReaderId<PlayerTerrainCollision>;
pub type OnPlayerPowerupCollisionReader = ReaderId<PlayerPowerupCollision>;
pub type OnMissileTerrainCollisionReader = ReaderId<MissileTerrainCollision>;
